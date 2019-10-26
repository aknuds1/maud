#![feature(test)]

extern crate test;

use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize, Debug)]
struct Entry {
    name: &'static str,
    score: u16,
}

static SOURCE: &'static str = "<html>
  <head>
    <title>{{ year }}</title>
  </head>
  <body>
    <h1>CSL {{ year }}</h1>
    <ul>
    {% for team in teams %}
      <li class=\"{% if loop.first %}champion{% endif %}\">
      <b>{{ team.name }}</b>: {{ team.score }}
      </li>
    {% endfor %}
    </ul>
  </body>
</html>";

#[bench]
fn render_template(b: &mut test::Bencher) {
    let mut tera = test::black_box(Tera::default());
    tera.add_raw_template("table", SOURCE).unwrap();

    let context = test::black_box({
        let mut context = Context::new();
        context.insert("teams", &[
            Entry { name: "Jiangsu", score: 43 },
            Entry { name: "Beijing", score: 27 },
            Entry { name: "Guangzhou", score: 22 },
            Entry { name: "Shandong", score: 12 },
        ]);
        context.insert("year", &"2015");
        context
    });

    b.iter(|| tera.render("table", &context));
}
