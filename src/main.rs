use lazy_static::lazy_static;

use regex::Regex;


use std::io;
use pgn_reader:: { RawHeader, RawComment, Visitor, Skip, BufferedReader, SanPlus };
use shakmaty::{Chess, Position};
use shakmaty::fen::Fen;

#[derive(Debug, Clone)]
struct FenEval {
    link: String,
    fen: String,
    eval: String,
    ply: usize,
}

struct GameWithEval {
    pos: Chess,
    ply: usize,
    link: String,
    evals: Vec<FenEval>
}

impl GameWithEval {
    fn new() -> GameWithEval {
        GameWithEval{ ply: 0, link: "".into(), pos: Chess::default(), evals: vec!() }
    }
}

impl Visitor for GameWithEval {
    type Result = Vec<FenEval>;


    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if key == b"Site" {
            std::str::from_utf8(value.as_bytes()).ok().map(|link| 
                self.link = link.into()
            );
        }
    }

    fn comment(&mut self, comment: RawComment) {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"%eval ([^\]]*)").unwrap();
        }

        let res = std::str::from_utf8(comment.as_bytes());

        res.ok().map( |res| {
            for cap in RE.captures_iter(res) {

                self.evals.push(FenEval {
                    eval: cap[1].into(),
                    ply: self.ply,
                    link: self.link.clone(),
                    fen: shakmaty::fen::fen(&self.pos)
                });
            }
        });

    }

    fn san(&mut self, _san_plus: SanPlus) {
        if let Ok(m) = _san_plus.san.to_move(&self.pos) {
            self.pos.play_unchecked(&m);
            self.ply+=1;
        } else {
            println!("Not")
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        ::std::mem::replace(&mut self.pos, Chess::default());

        self.ply = 0;
        println!("Here");
        println!("{}", self.evals.len());
        self.evals.clone()
    }
}

fn main() -> io::Result<()> {

    let pgn = b"

%eval 2.41] } 19. Nxe4 { [%eval 2.56] } 19... Qh6 { [%eval 2.97] } 20. Nxd[156/1804]
 2.98] } 20... cxd6?! { [%eval 3.79] } 21. d4?! { [%eval 2.92] } 21... Nf6 { [%eval
2.97] } 22. Rae1 { [%eval 2.84] } 22... Ng4?! { [%eval 3.34] } 23. h3 { [%eval 3.31]
 } 23... Nf6 { [%eval 3.16] } 24. Qf5 { [%eval 3.46] } 24... Rxe1?! { [%eval 4.2] }
25. Rxe1 { [%eval 4.03] } 25... g4? { [%eval 5.62] } 26. Bd3? { [%eval 3.89] } 26...
 gxh3? { [%eval 6.62] } 27. gxh3?? { [%eval 3.19] } 27... Kh8 { [%eval 3.25] } 28. R
e3?! { [%eval 2.48] } 28... Rg8+ { [%eval 2.79] } 29. Rg3 { [%eval 2.73] } 29... Rxg
3+ { [%eval 2.62] } 30. fxg3 { [%eval 2.68] } 30... Nh5?! { [%eval 3.5] } 31. g4? {
[%eval 2.37] } 31... Ng3?? { [%eval 5.83] } 32. Qf2 { [%eval 5.63] } 32... Qc1+ { [%
eval 5.73] } 33. Kg2 { [%eval 5.75] } 33... Qh1+? { [%eval 8.02] } 34. Kxg3 { [%eval
 8.84] } 34... Qf3+? { [%eval #7] } 35. Qxf3 { [%eval #6] } 1-0

        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut game_eval = GameWithEval::new();

    let _res: Vec<FenEval> = reader.into_iter(&mut game_eval).flat_map(|res| {
        if let Some(fens) = res.ok() {
            fens.clone()
        } else {
            Vec::new()
        }
    }).collect();

    println!("{:?}", _res);

    Ok(())


}
