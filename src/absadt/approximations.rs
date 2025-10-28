use super::enc::*;
use crate::common::{Cex as Model, *};
use crate::info::VarInfo;

pub(super) struct LinearApprox {
    /// Existing approx
    approx: Approx,
    // approx template
    // n_args * num of its approx
    coef: VarMap<VarMap<VarIdx>>,
    cnst: VarIdx,
    min: Option<i64>,
    max: Option<i64>,
}

impl Approximation for LinearApprox {
    fn apply(&self, arg_terms: &[Term]) -> Vec<Term> {
        let mut terms = self.approx.apply(arg_terms);
        let mut res = vec![term::var(self.cnst, typ::int())];
        let coefs = self.coef.iter().flatten();
        for (arg, coef) in arg_terms.into_iter().zip(coefs) {
            let t = term::mul(vec![term::var(*coef, typ::int()), arg.clone()]);
            res.push(t);
        }
        terms.push(term::add(res));
        terms
    }
}

impl LinearApprox {
    pub(super) fn constraint(&self) -> Option<Term> {
        let mut asserts = Vec::new();
        for c in self
            .coef
            .iter()
            .flatten()
            .chain(std::iter::once(&self.cnst))
        {
            if let Some(min) = self.min {
                let t = term::le(term::int(min), term::var(*c, typ::int()));
                asserts.push(t);
            }

            if let Some(max) = self.max {
                let t = term::le(term::var(*c, typ::int()), term::int(max));
                asserts.push(t);
            }
        }

        Some(term::and(asserts))
    }

    pub(super) fn instantiate(&self, model: &Model) -> Approx {
        let mut approx = self.approx.clone();
        log_debug!("{}-{} The old approximation is {approx}", file!(), line!());
        let cnst = &model[self.cnst];

        log_debug!("{}-{} the cnst is {cnst}", file!(), line!());
        let mut terms = vec![term::val(cnst.clone())];
        let mut constructor_args: Vec<usize> = Vec::with_capacity(approx.args.len());

        for (coef, arg) in self.coef.iter().flatten().zip(approx.args.iter()) {
            let val = &model[*coef];
            let val = term::val(val.clone());
            let var = term::var(arg.idx, arg.typ.clone());
            log_debug!("{}-{} val is {val}", file!(), line!());
            log_debug!("{}-{} var is {var}", file!(), line!());
            terms.push(term::mul(vec![val, var]));
        }
        let term = term::add(terms);
        approx.terms.push(term);
        log_debug!("{}-{} The new approximation is {approx}", file!(), line!());

        approx
    }

    pub(super) fn new(
        coef: VarMap<VarMap<VarIdx>>,
        fvs: &mut VarInfos,
        approx: Approx,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Self {
        let idx = fvs.next_index();
        let info = VarInfo::new("const_value".to_string(), typ::int(), idx);
        fvs.push(info);
        Self {
            coef,
            cnst: idx,
            approx,
            min,
            max,
        }
    }
}

pub(super) struct LinearIteApprox {
    approx: Approx,
    coef: VarMap<VarMap<VarIdx>>,
    cnst: VarIdx,
    min: Option<i64>,
    max: Option<i64>,
}

impl Approximation for LinearIteApprox {
    fn apply(&self, arg_terms: &[Term]) -> Vec<Term> {
        let mut terms = self.approx.apply(arg_terms);
        let mut res = vec![term::var(self.cnst, typ::int())];
        let coefs = self.coef.iter().flatten();
        for (arg, coef) in arg_terms.iter().zip(coefs) {
            let t = term::mul(vec![term::var(*coef, typ::int()), arg.clone()]);
            res.push(t);
        }
        terms.push(term::add(res));
        terms
    }
}

impl LinearIteApprox {
    pub(super) fn constraint(&self) -> Option<Term> {
        let mut asserts = Vec::new();
        for c in self
            .coef
            .iter()
            .flatten()
            .chain(std::iter::once(&self.cnst))
        {
            if let Some(min) = self.min {
                let t = term::le(term::int(min), term::var(*c, typ::int()));
                asserts.push(t);
            }

            if let Some(max) = self.max {
                let t = term::le(term::var(*c, typ::int()), term::int(max));
                asserts.push(t);
            }
        }

        Some(term::and(asserts))
    }

    pub(super) fn instantiate(&self, model: &Model) -> Approx {
        for (idx, elem) in self.coef.iter().enumerate() {
            log_debug!("self.coef[{idx}] = {elem}");
        }

        const N_NEW_TERMS: usize = 4;
        let mut approx = self.approx.clone();

        let cnst = &model[self.cnst];
        log_debug!("{}-{} the cnst is {cnst}", file!(), line!());
        let mut terms = Vec::with_capacity(N_NEW_TERMS);
        // We need boolean condition (2 terms)
        // the true branch (1 term)
        // The false branch (1 term)
        for _ in 0..N_NEW_TERMS {
            let mut args_approx = vec![term::val(cnst.clone())];
            for (coef, arg) in self.coef.iter().flatten().zip(approx.args.iter()) {
                log_debug!(
                    "{}-{} the arg is {arg} with coefficient {coef}",
                    file!(),
                    line!()
                );
                let val = &model[*coef];
                log_debug!("{}-{} first is {val}", file!(), line!());
                let val = term::val(val.clone());
                let var = term::var(arg.idx, arg.typ.clone());
                log_debug!("{}-{} second val is {val}", file!(), line!());
                log_debug!("{}-{} var is {var}", file!(), line!());
                args_approx.push(term::mul(vec![val, var.clone()]));
                log_debug!("{}-{} args_approx is {args_approx:?}", file!(), line!());
            }
            debug_assert_eq!(args_approx.len(), approx.args.len() + 1);
            terms.push(term::add(args_approx));
        }
        debug_assert_eq!(terms.len(), 4);
        let bool_condition = term::gt(terms[0].clone(), terms[1].clone());
        approx.terms.push(term::ite(
            bool_condition,
            terms[2].clone(),
            terms[3].clone(),
        ));
        log_debug!("{}-{} The new approximation is {approx}", file!(), line!());

        approx
    }

    pub(super) fn new(
        coef: VarMap<VarMap<VarIdx>>,
        fvs: &mut VarInfos,
        approx: Approx,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Self {
        let idx = fvs.next_index();
        let info = VarInfo::new("const_value".to_string(), typ::int(), idx);
        fvs.push(info);
        Self {
            coef,
            cnst: idx,
            approx,
            min,
            max,
        }
    }
}
