use room::build;
use room::prelude::*;
fn main() -> std::fmt::Result {
    let html = build::raw_escapable("<!DOCTYPE html>").chain(build::elem("html"));

    let style = build::elem("style").append(build::raw(
        "table, th, td {
        border: 1px solid black;
        border-collapse: collapse;
        animation: mymove 5s infinite;
      }
      @keyframes mymove {
          from {background-color: red;}
          to {background-color: blue;}
      }",
    ));

    let table = {
        let table = build::elem("table").with_attr(("style", format_move!("width:{}%", 100)));

        let rows = (0..20).map(|i| {
            let columns = chain!(
                build::elem("th").append(build::raw(format_move!("Hay {}:1", i))),
                build::elem("th").append(build::raw(format_move!("Hay {}:2", i))),
                build::elem("th").append(build::raw(format_move!("Hay {}:3", i)))
            );

            build::elem("tr").append(columns)
        });
        table.append(rows)
    };

    let all = html.append(style).append(table);

    let w = room::tools::upgrade_write(std::io::stdout());
    all.render_with(w)
}
