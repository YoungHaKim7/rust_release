struct FnCall<'a> {
    fn_name: &'a str,
    args: Vec<i32>,
}

fn is_legal_ident(s: &str) -> bool {
    s.chars()
        .all(|c| ('a'..='z').contains(&c) || ('A'..='Z').contains(&c))
}

#[feature(let_chains)]
impl<'a> FnCall<'a> {
    fn parse(s: &'a str) -> Option<Self> {
        if let Some((fn_name, after_name)) = s.split_once("(")
            && !fn_name.is_empty()
            && is_legal_ident(fn_name)
            && let Some((args_str, "")) = after_name.rsplit_once(")")
        {
            let args = args_str
                .split(',')
                .map(|arg| arg.parse())
                .collect::<Result<Vec<_>, _>>();
            args.ok().map(|args| FnCall { fn_name, args })
        } else {
            None
        }
    }
    fn exec(&self) -> Option<i32> {
        let iter = self.args.iter().copied();
        match self.fn_name {
            "sum" => Some(iter.sum()),
            "max" => iter.max(),
            "min" => iter.min(),
            _ => None,
        }
    }
}

fn main() {
    println!("{:?}", FnCall::parse("sum(1,2,3)").unwrap().exec());
    println!("{:?}", FnCall::parse("max(4,5)").unwrap().exec());
}
