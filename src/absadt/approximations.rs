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
        log!("{}-{} the `arg_terms` are {arg_terms:#?}", file!(), line!());
        log!("{}-{} The terms are {terms:#?}", file!(), line!());
        let mut res = vec![term::var(self.cnst, typ::int())];
        let coefs = self.coef.iter().flatten();
        for (arg, coef) in arg_terms.iter().zip(coefs) {
            let t = term::mul(vec![term::var(*coef, typ::int()), arg.clone()]);
            log!(
                "{}-{} the new multiplication between {arg} and {} is {t}",
                file!(),
                line!(),
                term::var(*coef, typ::int())
            );
            res.push(t);
        }
        terms.push(term::add(res));
        log!("{}-{} the term produced is {:?}", file!(), line!(), terms);
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
        let cnst = &model[self.cnst];

        let mut terms = vec![term::val(cnst.clone())];
        //let mut constructor_args: Vec<usize> = Vec::with_capacity(approx.args.len());

        for (coef, arg) in self.coef.iter().flatten().zip(approx.args.iter()) {
            let val = &model[*coef];
            let val = term::val(val.clone());
            let var = term::var(arg.idx, arg.typ.clone());
            terms.push(term::mul(vec![val, var]));
        }
        let term = term::add(terms);
        approx.terms.push(term);
        log!("{}-{} The new approximation is {approx}", file!(), line!());

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
    /// In a linear ITE the coefficent goes 4 by 4, so the first 4 coefficient are for the first element
    coef: VarMap<VarMap<VarIdx>>,
    cnst: VarIdx,
    min: Option<i64>,
    max: Option<i64>,
}

impl Approximation for LinearIteApprox {
    #[track_caller]
    fn apply(&self, arg_terms: &[Term]) -> Vec<Term> {
        const N_LINEAR_EXPRESSIONS: usize = 4;
        log!(
            "{}-{} Executing apply beacuse of {}",
            file!(),
            line!(),
            std::panic::Location::caller()
        );
        log!("{}-{} the `arg_terms` are {arg_terms:#?}", file!(), line!());
        let mut term = self.approx.apply(arg_terms);
        log!("{}-{} The terms are {term:#?}", file!(), line!());
        let mut catamorphis: [Term; N_LINEAR_EXPRESSIONS] = [
            term::var(self.cnst, typ::int()),
            term::var(self.cnst, typ::int()),
            term::var(self.cnst, typ::int()),
            term::var(self.cnst, typ::int()),
        ];
        for (arg_idx, arg) in arg_terms.iter().enumerate() {
            for (idx, coef) in self.coef.get(arg_idx).unwrap().to_vec().iter().enumerate() {
                log!(
                    "{}-{} {} += ({} * {})",
                    file!(),
                    line!(),
                    catamorphis[idx],
                    arg.clone(),
                    term::int_var(*coef)
                );
                catamorphis[idx] = term::add(vec![
                    catamorphis[idx].clone(),
                    term::mul(vec![arg.clone(), term::int_var(*coef)]),
                ]);
            }
        }
        term.push(term::ite(
            term::gt(catamorphis[0].clone(), catamorphis[1].clone()),
            catamorphis[2].clone(),
            catamorphis[3].clone(),
        ));
        log!("{}-{} the term produced is {:?}", file!(), line!(), term);
        term
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

    #[track_caller]
    pub(super) fn instantiate(&self, model: &Model) -> Approx {
        log!(
            "{}-{} the function instantiate has been called by {}",
            file!(),
            line!(),
            std::panic::Location::caller()
        );
        const N_NEW_TERMS: usize = 4;
        let mut approx = self.approx.clone();

        let cnst = &model[self.cnst];
        log!("{}-{} the cnst is {cnst:#?}", file!(), line!());
        let mut terms = Vec::with_capacity(N_NEW_TERMS);
        // We need boolean condition (2 terms)
        // the true branch (1 term)
        // The false branch (1 term)
        for _ in 0..N_NEW_TERMS {
            let mut args_approx = vec![term::val(cnst.clone())];
            for (coef, arg) in self.coef.iter().flatten().zip(approx.args.iter()) {
                let val = &model[*coef];
                let val = term::val(val.clone());
                let var = term::var(arg.idx, arg.typ.clone());
                log!("Multipling {val} with {var}");
                args_approx.push(term::mul(vec![val, var]));
            }
            terms.push(term::add(args_approx));
        }
        debug_assert_eq!(terms.len(), 4);
        let bool_condition = term::gt(terms[0].clone(), terms[1].clone());
        approx.terms.push(term::ite(
            bool_condition,
            terms[2].clone(),
            terms[3].clone(),
        ));
        log!("{}-{} The new approximation is {approx}", file!(), line!());

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
