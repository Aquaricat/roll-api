use die::*;
use rocket_contrib::{Json, Value};
use roll::*;
use std::time::Instant;
use ttml::arg::{Arg, ArgValue, ComparisonArg, RollArg};
use ttml::parser::parse_step_p;

#[derive(Serialize)]
pub struct RollsResponse {
    pub roll: Roll,
    pub execution_time: u64,
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource not found."
    }))
}

#[get("/<command>", format = "application/json")]
pub fn roll(command: String) -> Option<Json<RollsResponse>> {
    // Start the timer
    let start = Instant::now();

    // Parse the roll command as if we're passing it through TTML
    let input = "!roll ".to_string() + &command;
    let step_result = parse_step_p(input.as_bytes());

    // Successful parse, roll the die
    if step_result.is_err() == false {
        let (_, step) = step_result.unwrap();

        // Build a list of dice to roll
        let mut rolls: Vec<Roll> = vec![];

        // Build a list of flags
        let mut flags = RollFlags::new();

        // Loop through each step, push the dice when necessary
        for arg in &step.args {
            if let &Arg::Roll(RollArg::N(ArgValue::Number(n))) = arg {
                flags.n = n as i16;
                flags.equation = flags.equation + &n.to_string();
            } else if let &Arg::Roll(RollArg::D(ArgValue::Number(d))) = arg {
                flags.max = d as i16;
                flags.die = match d {
                    100   => DieType::D100,
                    20    => DieType::D20,
                    12    => DieType::D12,
                    10    => DieType::D10,
                    8     => DieType::D8,
                    6     => DieType::D6,
                    4     => DieType::D4,
                    _     => DieType::Other,
                };
                flags.equation = flags.equation + &"d" + &d.to_string();
            } else if let &Arg::Roll(RollArg::H(ArgValue::Number(h))) = arg {
                flags.kh = h as i16;
                flags.equation = flags.equation + &"kh" + &h.to_string();
            } else if let &Arg::Roll(RollArg::L(ArgValue::Number(l))) = arg {
                flags.kl = l as i16;
                flags.equation = flags.equation + &"kl" + &l.to_string();
            } else if let &Arg::Roll(RollArg::GT(ArgValue::Number(gt))) = arg {
                flags.gt = gt as u16;
                flags.equation = flags.equation + &"gt" + &gt.to_string();
            } else if let &Arg::Roll(RollArg::GTE(ArgValue::Number(gte))) = arg {
                flags.gte = gte as u16;
                flags.equation = flags.equation + &"gte" + &gte.to_string();
            } else if let &Arg::Roll(RollArg::LT(ArgValue::Number(lt))) = arg {
                flags.lt = lt as u16;
                flags.equation = flags.equation + &"lt" + &lt.to_string();
            } else if let &Arg::Roll(RollArg::LTE(ArgValue::Number(lte))) = arg {
                flags.lte = lte as u16;
                flags.equation = flags.equation + &"lte" + &lte.to_string();
            } else if let &Arg::Roll(RollArg::RR(ref comparitive)) = arg {
                flags.rr = match &comparitive.value {
                    &ArgValue::Number(n) => n as i16,
                    _ => 0
                };
                match comparitive.op {
                    ComparisonArg::GreaterThan => {
                        flags.rr_op = Some(ComparisonArg::GreaterThan);
                        flags.equation = flags.equation + &"rr>" + &flags.rr.to_string();
                    },
                    ComparisonArg::GreaterThanOrEqual => {
                        flags.rr_op = Some(ComparisonArg::GreaterThanOrEqual);
                        flags.equation = flags.equation + &"rr>=" + &flags.rr.to_string();
                    },
                    ComparisonArg::LessThan => {
                        flags.rr_op = Some(ComparisonArg::LessThan);
                        flags.equation = flags.equation + &"rr<" + &flags.rr.to_string();
                    },
                    ComparisonArg::LessThanOrEqual => {
                        flags.rr_op = Some(ComparisonArg::LessThanOrEqual);
                        flags.equation = flags.equation + &"rr<=" + &flags.rr.to_string();
                    },
                    ComparisonArg::EqualTo => {
                        flags.rr_op = Some(ComparisonArg::EqualTo);
                        flags.equation = flags.equation + &"rr==" + &flags.rr.to_string();
                    },
                };
            } else if let &Arg::Roll(RollArg::RO(ref comparitive)) = arg {
                flags.ro = match &comparitive.value {
                    &ArgValue::Number(n) => n as i16,
                    _ => 0
                };
                match comparitive.op {
                    ComparisonArg::GreaterThan => {
                        flags.ro_op = Some(ComparisonArg::GreaterThan);
                        flags.equation = flags.equation + &"ro>" + &flags.ro.to_string();
                    },
                    ComparisonArg::GreaterThanOrEqual => {
                        flags.ro_op = Some(ComparisonArg::GreaterThanOrEqual);
                        flags.equation = flags.equation + &"ro>=" + &flags.ro.to_string();
                    },
                    ComparisonArg::LessThan => {
                        flags.ro_op = Some(ComparisonArg::LessThan);
                        flags.equation = flags.equation + &"ro<" + &flags.ro.to_string();
                    },
                    ComparisonArg::LessThanOrEqual => {
                        flags.ro_op = Some(ComparisonArg::LessThanOrEqual);
                        flags.equation = flags.equation + &"ro<=" + &flags.ro.to_string();
                    },
                    ComparisonArg::EqualTo => {
                        flags.ro_op = Some(ComparisonArg::EqualTo);
                        flags.equation = flags.equation + &"ro==" + &flags.ro.to_string();
                    },
                };
            } else if let &Arg::Roll(RollArg::XF(ref comparitive)) = arg {
                flags.rr = match &comparitive.value {
                    &ArgValue::Number(n) => n as i16,
                    _ => 0
                };
                match comparitive.op {
                    ComparisonArg::GreaterThan => {
                        flags.xf_op = Some(ComparisonArg::GreaterThan);
                        flags.equation = flags.equation + &"xf>" + &flags.xf.to_string();
                    },
                    ComparisonArg::GreaterThanOrEqual => {
                        flags.xf_op = Some(ComparisonArg::GreaterThanOrEqual);
                        flags.equation = flags.equation + &"xf>=" + &flags.xf.to_string();
                    },
                    ComparisonArg::LessThan => {
                        flags.xf_op = Some(ComparisonArg::LessThan);
                        flags.equation = flags.equation + &"xf<" + &flags.xf.to_string();
                    },
                    ComparisonArg::LessThanOrEqual => {
                        flags.xf_op = Some(ComparisonArg::LessThanOrEqual);
                        flags.equation = flags.equation + &"xf<=" + &flags.xf.to_string();
                    },
                    ComparisonArg::EqualTo => {
                        flags.xf_op = Some(ComparisonArg::EqualTo);
                        flags.equation = flags.equation + &"xf==" + &flags.xf.to_string();
                    },
                };
            } else if let &Arg::Roll(RollArg::XO(ref comparitive)) = arg {
                flags.ro = match &comparitive.value {
                    &ArgValue::Number(n) => n as i16,
                    _ => 0
                };
                match comparitive.op {
                    ComparisonArg::GreaterThan => {
                        flags.xo_op = Some(ComparisonArg::GreaterThan);
                        flags.equation = flags.equation + &"xo>" + &flags.xo.to_string();
                    },
                    ComparisonArg::GreaterThanOrEqual => {
                        flags.xo_op = Some(ComparisonArg::GreaterThanOrEqual);
                        flags.equation = flags.equation + &"xo>=" + &flags.xo.to_string();
                    },
                    ComparisonArg::LessThan => {
                        flags.xo_op = Some(ComparisonArg::LessThan);
                        flags.equation = flags.equation + &"xo<" + &flags.xo.to_string();
                    },
                    ComparisonArg::LessThanOrEqual => {
                        flags.xo_op = Some(ComparisonArg::LessThanOrEqual);
                        flags.equation = flags.equation + &"xo<=" + &flags.xo.to_string();
                    },
                    ComparisonArg::EqualTo => {
                        flags.xo_op = Some(ComparisonArg::EqualTo);
                        flags.equation = flags.equation + &"xo==" + &flags.xo.to_string();
                    },
                };
            } else if let &Arg::Roll(RollArg::ModifierPos(ArgValue::Number(mp))) = arg {
                flags.modifiers.push(mp as i16);
                flags.equation = flags.equation + &"+" + &mp.to_string();
            } else if let &Arg::Roll(RollArg::ModifierNeg(ArgValue::Number(mn))) = arg {
                flags.modifiers.push(mn as i16 * -1);
                flags.equation = flags.equation + &"-" + &mn.to_string();
            } else if let &Arg::Roll(RollArg::Max(ArgValue::Number(max))) = arg {
                flags.max = max as i16;
                flags.equation = flags.equation + &"max" + &max.to_string();
            } else if let &Arg::Roll(RollArg::Min(ArgValue::Number(min))) = arg {
                flags.min = min as i16;
                flags.equation = flags.equation + &"min" + &min.to_string();
            } else if let &Arg::Roll(RollArg::Sides(ref r_sides)) = arg {
                let mut min = 0;
                let mut max = 0;
                let sides: Vec<i16> = r_sides.into_iter().map(|side| (
                    match side {
                        &ArgValue::Number(n) => {
                            if n < min || n == 0 {
                                min = n.clone()
                            } else if n > max {
                                max = n.clone()
                            }
                            n.clone() as i16
                        },
                        _ => 0 as i16
                    }
                )).collect();
                flags.sides = Some(sides.clone());
                flags.min = min as i16;
                flags.max = max as i16;
                let side_strs: Vec<String> = sides.into_iter().map(|side| (side.to_string())).collect();
                flags.equation = flags.equation + &"[" + &side_strs.join(",") + &"]";
            } else if let &Arg::Roll(RollArg::Comment(ArgValue::Text(ref comment))) = arg {
                flags.equation = flags.equation + &"[" + comment + &"]";
            } else if let &Arg::Roll(RollArg::Primitive(_)) = arg {
                // Execute this roll before starting the next one
                rolls.push(Roll::new(flags));

                // Reset the flags
                flags = RollFlags::new();
            }
        }

        // Build the final roll
        let roll = Roll::new(flags);
        // let original_equation = roll.equation.clone();

        // Take all the dice from previous rolls and append them to this roll
        // for mut r in rolls.into_iter() {
            // roll.dice.append(&mut r.dice);
            // roll.modifiers.append(&mut r.modifiers);
            // roll.value += r.value;
            // roll.raw_value += r.raw_value;
            // roll.equation = r.equation + " + ";
        // }
        // roll.equation = roll.equation + &original_equation;

        let elapsed = start.elapsed();
        let response = RollsResponse {
            roll,
            execution_time: (elapsed.as_secs() * 1000) + (elapsed.subsec_nanos() / 1000000) as u64,
        };
        Some(Json(response))
    } else {
        None
    }
}
