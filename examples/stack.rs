use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let all = build::from_stack(|stack| {
        let a = build::elem("a2");
        let b = build::elem("b2");
        let c = build::elem("c2").with_tab("→");

        let mut stack = stack.push(a)?.push(b)?.push(c)?;

        for i in 0..5 {
            let e = build::elem(format_move!("x2:{}", i)).inline();
            stack.put(e)?;
        }
        stack.pop()?.pop()?.pop()
    });

    tagu::render(all.with_tab(" "), tagu::stdout_fmt())
}
