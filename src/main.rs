#![feature(slice_concat_ext)]
use std::io;
use std::io::prelude::*;
use std::env;
use std::iter;
use std::slice::SliceConcatExt;

fn input() -> Vec<Vec<String>> {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut cols = None;

    // we cannot omit the 'res' variable. is it the limitation of rust type system?
    let res = lock.lines().map(|x| {
        let line = x.unwrap();
        let splitted: Vec<_> = line.split(',').map(|x| x.to_string()).collect();
        if let Some(cols) = cols {
            assert!(splitted.len() == cols);
        } else {
            cols = Some(splitted.len());
        }

        splitted
    }).collect();
    res
}

fn maxcols(cells: &Vec<Vec<String>>) -> Vec<usize> {
    let mut res = vec![0usize; cells[0].len()];
    for row in cells {
        for i in 0..row.len() {
            res[i] = std::cmp::max(res[i], row[i].len());
        }
    }
    res
}

// return: (blocks, transformed_table)
fn transform(cells: &Vec<Vec<String>>, line_to_wrap: usize) -> (usize, Vec<Vec<String>>) { // {{{
    let head = &cells[0];

    let orig_len = cells.len();
    let data_len = orig_len - 1;
    let cols = cells[0].len();

    let blocks = (data_len + (line_to_wrap - 1)) / line_to_wrap; // 切り上げ計算
    let mut result = vec![];
    result.push(iter::repeat(head.clone())
                .take(blocks)
                .collect::<Vec<_>>()
                .concat());
    for l in 0..line_to_wrap {
        let mut to_push = vec![];
        for i in 0..blocks {
            let orig = line_to_wrap * i + l + 1;
            let mut to_append = if orig < orig_len {
                cells[orig].clone()
            } else {
                vec!["".to_string(); cols]
            };
            to_push.append(&mut to_append);
        }
        result.push(to_push);
    }
    (blocks, result)
} // }}}

fn transform_maxwidth(maxwidth: Vec<usize>, blocks: usize) -> Vec<usize> {
    iter::repeat(maxwidth).take(blocks).collect::<Vec<_>>().concat()
}

fn hline(width: &Vec<usize>, padding: usize, delim: char) -> String { // {{{
    let line = width.iter().map(|w| {
        iter::repeat(delim)
            .take(*w + (padding * 2))
            .collect::<String>()
    }).collect::<Vec<String>>().join("+");
    format!("+{}+", line)
} // }}}

// maxwidth is not implemented.
fn draw(table: Vec<Vec<String>>, transformed_maxwidth: &Vec<usize>) -> String {
    let mut result_row = vec![];
    result_row.push(hline(transformed_maxwidth, 1, '-'));
    let mut first = true;
    for row in table {
        result_row.push(draw_row(row, transformed_maxwidth));
        let delim = if first { first = false; '=' } else { '-' };
        result_row.push(hline(transformed_maxwidth, 1, delim));
    }
    result_row.join("\n")
}

fn draw_row(row: Vec<String>, transformed_maxwidth: &Vec<usize>) -> String {
    let mut result = vec![];
    for (cell, w) in row.into_iter().zip(transformed_maxwidth.iter()) {
        result.push(draw_cell(cell, *w));
    }
    concat_cells_in_row(result, transformed_maxwidth)
}

fn draw_cell(cell: String, w: usize) -> Vec<String> {
    let mut chars = cell.chars();
    let mut result = vec![];
    loop {
        let mut end = false;
        let mut to_push = String::new();
        for _ in 0..w {
            let next = chars.next();
            if let Some(ch) = next {
                to_push.push(ch);
            } else {
                end = true;
                break;
            }
        }
        if &to_push != "" { result.push(to_push); }
        if end { break; }
    }
    if result.is_empty() {
        result.push("".to_string());
    }
    result
}

fn concat_cells_in_row(mut cells_in_row: Vec<Vec<String>>, transformed_maxwidth: &Vec<usize>) -> String {
    let mut maxheight = 0;
    for cell in &cells_in_row {
        maxheight = std::cmp::max(maxheight, cell.len());
    }
    for cell in &mut cells_in_row {
        cell.resize(maxheight, "".to_string());
    }

    let mut result = vec![];
    for h in 0..maxheight {
        result.push(format!("|{}|", cells_in_row.iter().zip(transformed_maxwidth.iter()).map(|(x,w)| {
            let disp = if h < x.len() {
                x[h].clone()
            } else {
                "".to_string()
            };
            format!(" {:>w$} ", disp, w=w)
        }).collect::<Vec<_>>().join("|")))
    }
    result.join("\n")
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} line_to_wrap [maxwidth]", args[0]);
        eprintln!("          line to wrap: line to wrap the table");
        eprintln!("          maxwidth: (optional) max width of each column");
        eprintln!("                    specify such as '1,4,2,3,4'");
        std::process::exit(1);
    }
    let line_to_wrap: usize = args[1].parse()
        .expect("failed to parse argument 1: line_to_wrap.");

    let cells = input();
    if cells.len() == 0 { return; }
    let maxcols = maxcols(&cells);
    let maxwidth: Vec<usize>;
    if args.len() >= 3 {
        maxwidth = args[2].split(',').map(|x| x.parse().expect("failed to parse maxwidth.")).collect();
    } else {
        maxwidth = maxcols;
    }
    assert!(maxwidth.len() == cells[0].len(), "maxwidth's size is not equal to the table's columns count.");

    let (blocks, table) = transform(&cells, line_to_wrap);
    let transformed_maxwidth = transform_maxwidth(maxwidth, blocks);
    println!("{}", draw(table, &transformed_maxwidth));
}
