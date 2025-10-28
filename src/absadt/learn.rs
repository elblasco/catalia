use super::chc::CEX;
use super::enc::*;
use crate::absadt::approximations::{LinearApprox, LinearIteApprox};
use crate::common::{smt::FullParser as Parser, Cex as Model, *};
use crate::info::VarInfo;
use crate::term::typ::RTyp;

const CONSTRAINT_CHECK_TIMEOUT: usize = 1;

struct TemplateInfo {
    parameters: VarInfos,
    encs: BTreeMap<Typ, Enc<Template>>,
}

impl TemplateInfo {
    fn define_constraints(&self, solver: &mut Solver<Parser>) -> Res<()> {
        let constrs = if let Some(constrs) = self.constraint() {
            constrs
        } else {
            return Ok(());
        };

        writeln!(solver, "; Constraints on template variables")?;
        for c in constrs {
            writeln!(solver, "(assert {})", c)?;
        }
        writeln!(solver)?;

        Ok(())
    }
    /// Define paramter constants
    fn define_parameters(&self, solver: &mut Solver<Parser>) -> Res<()> {
        for var in self.parameters.iter() {
            solver.declare_const(format!("v_{}", var.idx), var.typ.to_string())?;
        }
        Ok(())
    }

    fn new_linear_approx(
        encs: BTreeMap<Typ, Encoder>,
        min: Option<i64>,
        max: Option<i64>,
    ) -> TemplateInfo {
        let mut fvs = VarInfos::new();

        let mut new_encs = BTreeMap::new();

        // prepare LinearApprox for each constructor
        for (typ, enc) in encs.iter() {
            let mut approxs = BTreeMap::new();
            for constr in typ.dtyp_inspect().unwrap().0.news.keys() {
                let (ty, prms) = typ.dtyp_inspect().unwrap();
                let mut coefs = VarMap::new();
                // each constructor has a set of selectors
                for (sel, ty) in ty.selectors_of(constr).unwrap().iter() {
                    let ty = ty.to_type(Some(prms)).unwrap();
                    let n = match encs.get(&ty) {
                        Some(enc_for_ty) => {
                            // prepare template coefficients for all the approximations of the argument
                            enc_for_ty.n_params + 1
                        }
                        None => {
                            assert!(ty.is_int());
                            1
                        }
                    };
                    let name = format!("{constr}-{sel}");
                    // prepare coefs for constr-sel, which involes generating new template variables manged
                    // at the top level (`fvs`)
                    let args = prepare_coefs(name, &mut fvs, n);
                    coefs.push(args);
                }

                let mut approx = enc.approxs.get(constr).unwrap().clone();
                let n_args: usize = coefs
                    .iter()
                    .map(|x| x.iter().map(|_| 1).sum::<usize>())
                    .sum();
                // insert dummy variables for newly-introduced approximated integers
                for _ in 0..(n_args - approx.args.len()) {
                    approx.args.push(VarInfo::new(
                        format!("tmp-{}", approx.args.next_index()),
                        typ::int(),
                        approx.args.next_index(),
                    ));
                }
                approxs.insert(
                    constr.to_string(),
                    Template::Linear(LinearApprox::new(coefs.into(), &mut fvs, approx, min, max)),
                );
            }
            let enc = Enc {
                approxs,
                typ: typ.clone(),
                n_params: enc.n_params + 1,
            };
            new_encs.insert(typ.clone(), enc);
        }

        TemplateInfo {
            parameters: fvs,
            encs: new_encs,
        }
    }

    /// Instantiate a new Linear Ite approximation, not refinement
    fn new_linear_ite_approx(
        encs: BTreeMap<Typ, Encoder>,
        min: Option<i64>,
        max: Option<i64>,
    ) -> TemplateInfo {
        let mut fvs = VarInfos::new();

        let mut new_encs = BTreeMap::new();

        // prepare LinearIteApprox for each constructor
        for (typ, enc) in encs.iter() {
            log_debug!("The type is {typ} with encoding {enc}");
            let mut approxs = BTreeMap::new();
            for constr in typ.dtyp_inspect().unwrap().0.news.keys() {
                log_debug!("The constructor is {constr}");
                let (ty, prms) = typ.dtyp_inspect().unwrap();
                log_debug!("Pramater {prms:?}: {ty}");
                let mut coefs = VarMap::new();
                // each constructor has a set of selectors
                log_debug!(
                    "The list of all selectors is {:?}",
                    ty.selectors_of(constr).unwrap()
                );
                for (sel, ty) in ty.selectors_of(constr).unwrap() {
                    let local_approx = enc.approxs.get(constr);

                    log_debug!("The current argument is {sel}: {ty}");
                    let ty = ty.to_type(Some(prms)).unwrap();
                    log_debug!("The type proposed is {ty} with prms {prms:?}");
                    log_debug!("{encs:?}");
                    let approx_argument = match local_approx {
                        None => panic!("There should be an approximation for {constr}"),
                        Some(approx) => approx
                            .args
                            .iter()
                            .find(|elem| elem.typ == ty && elem.active),
                    };
                    //log_debug!("{approx_argument:?}");

                    // if let Some(encoder) = encs.get(&ty) {
                    // 	printl;m
                    // }

                    let n = match encs.get(&ty) {
                        Some(enc_for_ty) => {
                            log_debug!(
                                "{}-{} The encoding for {ty} is {enc_for_ty:?}",
                                file!(),
                                line!()
                            );
                            // prepare template coefficients for all the approximations of the argument
                            enc_for_ty.n_params + 1
                        }
                        None => {
                            log_debug!(
                                "{}-{} There is no encoding for type {ty} in {encs:?}",
                                file!(),
                                line!()
                            );
                            assert!(ty.is_int() || ty.is_bool());
                            1
                        }
                    };
                    let name = format!("{constr}-{sel}");
                    // prepare coefs for constr-sel, which involes generating new template variables manged
                    // at the top level (`fvs`)
                    let args = prepare_coefs(name, &mut fvs, n);
                    log_debug!("{}-{} the coefs array is {coefs}", file!(), line!());
                    log_debug!("{}-{} the ty is {ty}", file!(), line!());
                    log_debug!("{}-{} the n is {n}", file!(), line!());
                    log_debug!("{}-{} the args are {args}", file!(), line!());
                    coefs.push(args);
                }

                let mut approx = enc.approxs.get(constr).unwrap().clone();
                log_debug!("The aproximation is {approx}");
                let n_args: usize = coefs
                    .iter()
                    .map(|x| x.iter().map(|_| 1).sum::<usize>())
                    .sum();
                // insert dummy variables for newly-introduced approximated integers
                for _ in 0..(n_args - approx.args.len()) {
                    approx.args.push(VarInfo::new(
                        format!("tmp-{}", approx.args.next_index()),
                        typ::int(),
                        approx.args.next_index(),
                    ));
                }
                approxs.insert(
                    constr.to_string(),
                    Template::LinearIte(LinearIteApprox::new(coefs, &mut fvs, approx, min, max)),
                );
            }
            let enc = Enc {
                approxs,
                typ: typ.clone(),
                n_params: enc.n_params + 1,
            };
            new_encs.insert(typ.clone(), enc);
        }

        TemplateInfo {
            parameters: fvs,
            encs: new_encs,
        }
    }

    fn instantiate(&self, model: &Model) -> BTreeMap<Typ, Encoder> {
        self.encs
            .iter()
            .map(|(k, v)| (k.clone(), v.instantiate(model)))
            .collect()
    }

    fn constraint(&self) -> Option<Vec<Term>> {
        let mut asserts = Vec::new();
        for enc in self.encs.values() {
            for approx in enc.approxs.values() {
                if let Some(constr) = approx.constraint() {
                    asserts.push(constr);
                }
            }
        }
        Some(asserts)
    }
}

/// Controls
///   1. which Template to use
///     - including their parameters
///   2. which range of the existing approximations to use
struct TemplateScheduler {
    idx: usize,
    enc: BTreeMap<Typ, Encoder>,
}

enum TemplateType {
    BoundLinear { min: i64, max: i64 },
    Linear,
    LinearIte { min: i64, max: i64 },
}

impl std::fmt::Display for TemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemplateType::BoundLinear { min, max } => write!(f, "BoundLinear({}, {})", min, max),
            TemplateType::Linear => write!(f, "Linear"),
            TemplateType::LinearIte { min, max } => write!(f, "LinearIte({}, {})", min, max),
        }
    }
}

struct TemplateSchedItem {
    n_encs: usize,
    typ: TemplateType,
}

impl std::fmt::Display for TemplateSchedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (n_encs = {})", self.typ, self.n_encs)
    }
}

impl Approx {
    fn restrict_approx<I: Iterator<Item = Typ>>(
        &self,
        cur_enc: &BTreeMap<Typ, Encoder>,
        typs: I,
        n_encs: usize,
    ) -> Approx {
        // 1. original signature
        // 2. append arguments with n_encs
        // 3. append terms with n_encs
        for (typ, encods) in cur_enc {
            log_debug!(
                "{}-{} In Approx::restrict_approx up until now the encoding for {typ} is {encods:?}",
                file!(),
                line!()
            );
        }
        let mut new_args = VarInfos::new();
        for t in typs {
            log_debug!(
                "{}-{} In Approx::restrict_approx analysing type {t}",
                file!(),
                line!()
            );
            match cur_enc.get(&t) {
                Some(_) => {
                    for _ in 0..n_encs {
                        new_args.push(VarInfo::new(
                            format!("tmp-{}", new_args.next_index()),
                            typ::int(),
                            new_args.next_index(),
                        ));
                    }
                }
                None => {
                    new_args.push(VarInfo::new(
                        format!("tmp-{}", new_args.next_index()),
                        t.clone(),
                        new_args.next_index(),
                    ));
                }
            }
        }
        let new_terms = self.terms.iter().take(n_encs).cloned().collect();
        Approx {
            args: new_args,
            terms: new_terms,
        }
    }
}

impl Encoder {
    fn restrict_approx(
        &self,
        cur_enc: &BTreeMap<Typ, Encoder>,
        typ: &Typ,
        n_encs: usize,
    ) -> Encoder {
        let (ty, params) = typ.dtyp_inspect().unwrap();
        log_debug!(
            "{}-{} I am trying to approximate {ty} with paramters {params:?}",
            file!(),
            line!()
        );
        let mut new_approxs = BTreeMap::new();
        for (cnstr, args) in ty.news.iter() {
            log_debug!(
                "{}-{} in Encoder::restrict_approx analysing ({cnstr},{args:?})",
                file!(),
                line!()
            );
            let approx = self.approxs.get(cnstr).unwrap();
            let approx = approx.restrict_approx(
                cur_enc,
                args.iter().map(|(_, t)| t.to_type(Some(params)).unwrap()),
                n_encs,
            );
            new_approxs.insert(cnstr.clone(), approx);
        }
        Encoder {
            approxs: new_approxs,
            typ: self.typ.clone(),
            n_params: n_encs,
        }
    }
}

impl TemplateScheduler {
    const N_TEMPLATES: usize = 3; //10;

    const TEMPLATE_SCHDEDULING: [TemplateSchedItem; Self::N_TEMPLATES] = [
        TemplateSchedItem {
            n_encs: 1,
            typ: TemplateType::BoundLinear { min: -1, max: 1 },
        },
        TemplateSchedItem {
            n_encs: 1,
            typ: TemplateType::LinearIte { min: -1, max: 1 },
        },
        // TemplateSchedItem {
        //     n_encs: 2,
        //     typ: TemplateType::BoundLinear { min: -1, max: 1 },
        // },
        TemplateSchedItem {
            n_encs: 2,
            typ: TemplateType::LinearIte { min: -1, max: 1 },
        },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::BoundLinear { min: -1, max: 1 },
        // },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::LinearIte { min: -1, max: 1 },
        // },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::BoundLinear { min: -2, max: 2 },
        // },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::BoundLinear { min: -4, max: 4 },
        // },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::BoundLinear { min: -32, max: 32 },
        // },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::BoundLinear { min: -64, max: 64 },
        // },
        // TemplateSchedItem {
        //     n_encs: 3,
        //     typ: TemplateType::Linear,
        // },
    ];

    fn new(enc: BTreeMap<Typ, Encoder>) -> Self {
        Self { idx: 0, enc }
    }

    fn restrict_approx(&self, n_encs: usize) -> BTreeMap<Typ, Encoder> {
        let ret = self
            .enc
            .iter()
            .map(|(k, enc)| {
                let enc = enc.restrict_approx(&self.enc, k, n_encs);
                (k.clone(), enc)
            })
            .collect();
        log_debug!(
            "{}-{} The restricted encoding with {n_encs} encodings gave {ret:#?}",
            file!(),
            line!()
        );
        ret
    }
}

impl std::iter::Iterator for TemplateScheduler {
    type Item = TemplateInfo;
    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        log_debug!(
            "{}-{} The caller was {}",
            file!(),
            line!(),
            core::panic::Location::caller()
        );
        'a: loop {
            if self.idx >= Self::N_TEMPLATES {
                return None;
            }
            let next_template = &Self::TEMPLATE_SCHDEDULING[self.idx];
            self.idx += 1;

            for (_, v) in self.enc.iter() {
                // case where the next_template is too large
                if v.n_params + 1 < next_template.n_encs {
                    continue 'a;
                }
            }

            log_debug!(
                "{}-{} in here the encoding is {:?}",
                file!(),
                line!(),
                self.enc
            );

            let enc = self.restrict_approx(next_template.n_encs - 1);

            log_debug!(
                "{}-{} after the restrict appproximation the encoding is {enc:?}",
                file!(),
                line!()
            );

            let r = match next_template.typ {
                TemplateType::BoundLinear { min, max } => {
                    TemplateInfo::new_linear_approx(enc, Some(min), Some(max))
                }
                TemplateType::Linear => TemplateInfo::new_linear_approx(enc, None, None),
                TemplateType::LinearIte { min, max } => {
                    TemplateInfo::new_linear_ite_approx(enc, Some(min), Some(max))
                }
            };
            log_info!("Template: {}", next_template);
            break Some(r);
        }
    }
}

pub struct LearnCtx<'a> {
    original_encs: &'a mut BTreeMap<Typ, Encoder>,
    cex: &'a CEX,
    solver: &'a mut Solver<Parser>,
    profiler: &'a Profiler,
    models: Vec<Model>,
}

enum Template {
    Linear(LinearApprox),
    LinearIte(LinearIteApprox),
}

impl Approximation for Template {
    fn apply(&self, arg_terms: &[Term]) -> Vec<Term> {
        match self {
            Template::Linear(approx) => approx.apply(arg_terms),
            Template::LinearIte(approx) => approx.apply(arg_terms),
        }
    }
}

impl Template {
    fn instantiate(&self, model: &Model) -> Approx {
        match self {
            Template::Linear(approx) => approx.instantiate(model),
            Template::LinearIte(approx) => approx.instantiate(model),
        }
    }
    fn constraint(&self) -> Option<Term> {
        match self {
            Template::Linear(approx) => approx.constraint(),
            Template::LinearIte(approx) => approx.constraint(),
        }
    }
}

impl Enc<Template> {
    fn instantiate(&self, model: &Model) -> Encoder {
        let mut approxs = BTreeMap::new();
        for (constr, approx) in self.approxs.iter() {
            let approx = approx.instantiate(model);
            approxs.insert(constr.clone(), approx);
        }
        Encoder {
            approxs,
            typ: self.typ.clone(),
            n_params: self.n_params,
        }
    }
}

#[test]
fn test_linear_approx_apply() {
    let mut fvs = VarInfos::new();
    // dtyp = Cons(x)
    let coef = prepare_coefs("dtyp-cons", &mut fvs, 1);
    let approx = LinearApprox::new(vec![coef].into(), &mut fvs, Approx::empty(), None, None);

    let x = term::val(val::int(4));
    let argss = vec![x.clone()];
    let mut t = approx.apply(&argss);

    assert_eq!(t.len(), 1);
    let t = t.remove(0);

    let t2 = term::add(vec![
        term::var(1, typ::int()),
        term::mul(vec![term::var(0, typ::int()), x.clone()]),
    ]);
    println!("t: {}", t);
    println!("t2: {}", t2);
    for (a, b) in [(4i64, 3i64), (1, 2), (-4, 0)].into_iter() {
        let subst: VarHMap<_> = (0..2)
            .map(|x| VarIdx::from(x))
            .zip(vec![term::val(val::int(a)), term::val(val::int(b))].into_iter())
            .collect();
        assert_eq!(
            t.subst_total(&subst).unwrap().0.as_val(),
            t2.subst_total(&subst).unwrap().0.as_val()
        );
    }
}

fn prepare_coefs<S>(varname: S, fvs: &mut VarInfos, n: usize) -> VarMap<VarIdx>
where
    S: AsRef<str>,
{
    let varname = varname.as_ref();
    let mut res = VarMap::new();
    for i in 0..n {
        let idx = fvs.next_index();
        let info = VarInfo::new(format!("{varname}-{i}"), typ::int(), idx);
        res.push(idx);
        fvs.push(info);
    }
    res
}

impl<'a> LearnCtx<'a> {
    pub fn new(
        encs: &'a mut BTreeMap<Typ, Encoder>,
        cex: &'a CEX,
        solver: &'a mut Solver<Parser>,
        profiler: &'a Profiler,
    ) -> Self {
        let models = Vec::new();

        LearnCtx {
            original_encs: encs,
            cex,
            solver,
            models,
            profiler,
        }
    }

    /// Define encoding functions
    ///
    /// Assumption: Data types are all defined.
    fn define_enc_funs(&mut self) -> Res<()> {
        let ctx = super::enc::EncodeCtx::new(self.original_encs);
        let mut funs = Vec::new();

        for enc in self.original_encs.values() {
            enc.generate_enc_fun(&ctx, &mut funs)?;
        }

        let funs_strs = funs.into_iter().map(|(funname, ty, term)| {
            let args = vec![("v_0", ty.to_string())];
            let body = term.to_string();
            (funname, args, "Int", body)
        });
        self.solver.define_funs_rec(funs_strs)?;
        Ok(())
    }

    /// Define data types
    fn define_datatypes(&mut self) -> Res<()> {
        dtyp::write_all(&mut self.solver, "")?;
        Ok(())
    }

    fn get_model(&mut self, timeout: Option<usize>) -> Res<Option<Model>> {
        self.solver.reset()?;
        self.define_datatypes()?;
        self.define_enc_funs()?;
        self.cex
            .define_assert_with_enc(self.solver, self.original_encs)?;
        if let Some(tmo) = timeout {
            self.solver.set_option(":timeout", format!("{}000", tmo))?;
        } else {
            self.solver.set_option(":timeout", "4294967295")?;
        }
        let b = self.solver.check_sat()?;
        if !b {
            return Ok(None);
        }
        let model = self.solver.get_model()?;
        let model = Parser.fix_model(model)?;
        for elem in &model {
            log_debug!(
                "{}-{} variable {}, hashconsed type {} and hashconsed val {}",
                file!(),
                line!(),
                elem.0,
                elem.1,
                elem.2
            );
        }
        let cex = Model::of_model(&self.cex.vars, model, true)?;
        Ok(Some(cex))
    }

    fn get_template_model(
        &mut self,
        form: &term::Term,
        template_info: &TemplateInfo,
    ) -> Res<Option<Model>> {
        self.solver.reset()?;
        self.solver.set_option(":timeout", "4294967295")?;
        template_info.define_parameters(self.solver)?;
        template_info.define_constraints(self.solver)?;

        writeln!(self.solver, "; Target term")?;
        writeln!(self.solver, "(assert {})", form)?;

        writeln!(self.solver)?;
        let b = self.solver.check_sat()?;
        if !b {
            return Ok(None);
        }
        let model = self.solver.get_model()?;
        let model = Parser.fix_model(model)?;
        let cex = Model::of_model(&template_info.parameters, model, true)?;
        log_debug!("{}-{} the counter-example is {cex}", file!(), line!());
        Ok(Some(cex))
    }

    fn get_instantiation(
        &mut self,
        template_info: TemplateInfo,
    ) -> Res<Option<BTreeMap<Typ, Encoder>>> {
        // 1. Let l1, ..., lk be li in fv(cex)
        // 2. vis = [[m[li] for m in self.models] for li in l1, ..., lk]
        // 4. Declare a1, ... ak in coef(enc) as free variables
        // 5. Declare template functions for each dtype <- not needed?
        // form <- T
        // 6. for vi in vis:
        //    r <- cex
        //    for li in l1, ..., lk:
        //       r <- r.subst(li, enc.encode(vi))
        //    form <- form /\ r
        // 7. solve form and return the model for
        // return form

        // templates encoder
        let mut form = Vec::new();
        let encoder = EncodeCtx::new(&template_info.encs);
        for m in self.models.iter() {
            log_debug!("{}-{} `m` is {m:?}", file!(), line!());
            let mut terms =
                encoder.encode(&term::not(self.cex.term.clone()), &|_: &Typ, v: &VarIdx| {
                    let v = &m[*v];
                    encoder.encode_val(v)
                });
            log_debug!("{}-{} Terms is {terms:?}", file!(), line!());
            form.append(&mut terms)
        }
        // solve the form
        let form = term::and(form);
        log_debug!("cex encoded with template");
        log_debug!("{form}");

        let r = match self.get_template_model(&form, &template_info) {
            Err(e) => panic!("{e}"),
            Ok(ok) => ok.map(|m| {
                let encs = template_info.instantiate(&m);
                log_debug!("{}-{} The new encoding is {encs:?}", file!(), line!());
                encs
            }),
        };

        // let r = self
        //     .get_template_model(&form, &template_info)
        //     .is_err_and(|err| panic!("{err}"));

        // r.map(|m| {
        //     let encs = template_info.instantiate(&m);
        //     log_debug!("{}-{} The new encoding is {encs:?}", file!(), line!());
        //     encs
        // });
        Ok(r)
    }

    fn refine_enc(
        &mut self,
        original_encs: &BTreeMap<Typ, Encoder>,
    ) -> Res<Option<BTreeMap<Typ, Encoder>>> {
        for template_info in TemplateScheduler::new(original_encs.clone()) {
            match self.get_instantiation(template_info)? {
                None => {
                    log_debug!(
                        "{}-{} did not manage to instantiate a new encoding",
                        file!(),
                        line!()
                    );
                    continue;
                }
                Some(new_encs) => {
                    log_debug!(
                        "{}-{} managed to instantiate a new encoding, which is {new_encs:?}",
                        file!(),
                        line!()
                    );
                    return Ok(Some(new_encs));
                }
            }
        }
        Ok(None)
    }

    pub fn work(&mut self) -> Res<()> {
        // We now only consider the linear models
        // Appendinx them to the existing encodings
        let original_enc = self.original_encs.clone();
        log_debug!(
            "{}-{} The encoding before checking the counter-example is {original_enc:?}",
            file!(),
            line!()
        );
        let mut first = true;
        loop {
            // 1. Check if the new encoding can refute the counterexample
            log_info!("checking enc refutes cex...");
            if conf.split_step {
                pause("go?", self.profiler);
            }

            let timeout = if first {
                first = false;
                None
            } else {
                Some(CONSTRAINT_CHECK_TIMEOUT)
            };
            match self.get_model(timeout) {
                // The current cex is refuted
                Ok(None) => {
                    log_info!("Yes.");
                    break;
                }
                Ok(Some(model)) => {
                    log_info!("No.");
                    log_debug!("model: {}", model);

                    #[cfg(debug_assertions)]
                    {
                        for (idx, m) in self.models.iter().enumerate() {
                            log_debug!("A part of the model is {m}");
                            assert_ne!(m, &model, "model is duplicated");
                        }
                    }
                    self.models.push(model);
                }
                Err(e) if e.is_timeout() || e.is_unknown() => {
                    log_info!("Timeout or unknown");
                    break;
                }
                Err(e) => {
                    println!("err: {}", e);
                    return Err(e);
                }
            }
            // 2. If not, learn something new
            *self.original_encs = self
                .refine_enc(&original_enc)?
                .expect("No appropriate template found");

            log_debug!("new_encs: ");
            for (k, v) in self.original_encs.iter() {
                log_debug!("{}: {}", k, v);
            }
        }
        Ok(())
    }
}

/// Entrypoint for the learning algorithm
///
/// If this function returns Ok(()), some encodings are appended to `encs`
/// so that `cex` can be refuted.
pub fn work<'a>(
    encs: &'a mut BTreeMap<Typ, Encoder>,
    cex: &'a CEX,
    solver: &mut Solver<Parser>,
    profiler: &Profiler,
) -> Res<()> {
    log_debug!("The cex is {cex}");
    for enc in encs.iter() {
        log_debug!("The encoding used are {enc:?}");
    }
    let mut learn_ctx = LearnCtx::new(encs, cex, solver, profiler);
    learn_ctx.work()?;
    Ok(())
}
