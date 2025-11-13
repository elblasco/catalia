use super::super::enc::*;
use crate::common::{Cex as Model, *};
use crate::info::VarInfo;

pub struct BTreeSortApprox {
    /// In approx the number of arguments should be equal to the number of constructor arguments
    approx: Approx,
    coef: VarMap<VarMap<VarIdx>>,
    cnst: VarIdx,
    min: Option<i64>,
    max: Option<i64>,
}

impl Approximation for BTreeSortApprox {
    fn apply(&self, arg_terms: &[Term]) -> Vec<Term> {
        let mut term = self.approx.apply(arg_terms);
        let mut catamorphis: [Term; Self::ITE_PARTS] = [
            term::var(self.cnst, typ::int()),
            term::var(self.cnst, typ::int()),
            term::var(self.cnst, typ::int()),
            term::var(self.cnst, typ::int()),
        ];
        for (arg_idx, arg) in arg_terms.iter().enumerate() {
            for (idx, coef) in self.coef.get(arg_idx).unwrap().to_vec().iter().enumerate() {
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
        term
    }
}

impl BTreeSortApprox {
    /// How many parts form an ite expression:
    /// 1. lhs of boolean expression
    /// 2. rhs of boolean expression
    pub const ITE_PARTS: usize = 4;

    pub(crate) fn constraint(&self) -> Option<Term> {
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

    pub(crate) fn instantiate(&self, model: &Model) -> Approx {
        let mut approx = self.approx.clone();
        let cnst = term::val(model[self.cnst].clone());
        let mut terms = [cnst.clone(), cnst.clone(), cnst.clone(), cnst.clone()];

        let arguments_vec = approx.args.to_vec();
        for (idx, coef) in self.coef.iter().flatten().enumerate() {
            let arg_idx = idx.div_euclid(Self::ITE_PARTS);
            let term_idx = idx % Self::ITE_PARTS;
            let val = &model[*coef];
            let val = term::val(val.clone());
            let argument = arguments_vec.get(arg_idx).unwrap();
            let var = term::var(argument.idx, argument.typ.clone());
            terms[term_idx] = term::add(vec![terms[term_idx].clone(), term::mul(vec![val, var])]);
        }

        let bool_condition = term::gt(terms[0].clone(), terms[1].clone());
        approx.terms.push(term::ite(
            bool_condition,
            terms[2].clone(),
            terms[3].clone(),
        ));

        approx
    }

    pub(crate) fn new(
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
