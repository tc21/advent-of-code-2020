/* --- Part Two ---

The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)

For example, suppose you have the same list of bus IDs as above:

7,13,x,x,59,x,31,19

An x in the schedule means there are no constraints on what bus IDs must depart at that time.

This means you are looking for the earliest timestamp (called t) such that:

    Bus ID 7 departs at timestamp t.
    Bus ID 13 departs one minute after timestamp t.
    There are no requirements or restrictions on departures at two or three minutes after timestamp t.
    Bus ID 59 departs four minutes after timestamp t.
    There are no requirements or restrictions on departures at five minutes after timestamp t.
    Bus ID 31 departs six minutes after timestamp t.
    Bus ID 19 departs seven minutes after timestamp t.

The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.

In this example, the earliest timestamp at which this occurs is 1068781:

time     bus 7   bus 13  bus 59  bus 31  bus 19
1068773    .       .       .       .       .
1068774    D       .       .       .       .
1068775    .       .       .       .       .
1068776    .       .       .       .       .
1068777    .       .       .       .       .
1068778    .       .       .       .       .
1068779    .       .       .       .       .
1068780    .       .       .       .       .
1068781    D       .       .       .       .
1068782    .       D       .       .       .
1068783    .       .       .       .       .
1068784    .       .       .       .       .
1068785    .       .       D       .       .
1068786    .       .       .       .       .
1068787    .       .       .       D       .
1068788    D       .       .       .       D
1068789    .       .       .       .       .
1068790    .       .       .       .       .
1068791    .       .       .       .       .
1068792    .       .       .       .       .
1068793    .       .       .       .       .
1068794    .       .       .       .       .
1068795    D       D       .       .       .
1068796    .       .       .       .       .
1068797    .       .       .       .       .

In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.

Here are some other examples:

    The earliest timestamp that matches the list 17,x,13,19 is 3417.
    67,7,59,61 first occurs at timestamp 754018.
    67,x,7,59,61 first occurs at timestamp 779210.
    67,7,x,59,61 first occurs at timestamp 1261476.
    1789,37,47,1889 first occurs at timestamp 1202161486.

However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!

What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
*/
const INPUT: &str = include_str!("input");

fn main() {
    // Note: this is a complex math problem, which requires a bit of explanation.
    // I did not solve this myself. I used the solution to the equivalent problem
    // here: http://mathforum.org/library/drmath/view/51595.html
    let buses = INPUT.lines().nth(1).unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(index, split)| split.parse::<i64>().ok().map(|id| (id, -(index as i64))));

    let (_, solution) = buses.fold((1, 0), diophantus_combine);

    println!("{:?}", solution);
}

/// given two functions: f(x) = ax + b, g(x) = cx + d (represented by
/// x_args = (a, b) and y_args = (c, d)), returns a function: h(x) = kx + l
/// (represented by (k, l)).
///
/// The returned function h(x) satisfies:
///     for any integer i, there exists integers m, n such that h(i) = f(m) = g(n)
///
/// Effectively, h(x) generates integer solutions to f(x) = g(y), and then returns
/// not x or y, but f(x)
fn diophantus_combine(x_args: (i64, i64), y_args: (i64, i64)) -> (i64, i64) {
    // Sorry for the confusing variable names: this was a math problem
    let (x1, x0) = x_args;
    let (y1, y0) = y_args;

    // println!("solving {}x + {} = {}y + {}", x1, x0, y1, y0);

    let x_ = x1;
    let y_ = -y1;
    let c = y0 - x0;

    // println!("rewriting into {}x + {}y = {}", x_, y_, c);

    let (x_comb, y_comb) = euclid_combinators(x_, y_);
    let gcd = x_comb * x_ + y_comb * y_;

    // println!("found gcd({}, {}) = {}*{} + {}*{} = {}", x_, y_, x_comb, x_, y_comb, y_, gcd);

    // we can get the general solution x = x_comb + (y_ / gcd) * t

    let multiplicand = c / gcd;
    let x_solution = x_comb * multiplicand;

    let t0 = x_solution;
    let t1 = y_ / gcd;

    // this isn't needed, but to make the results "pretty"
    let t1 = t1.abs();
    let t0 = t0.rem_euclid(t1);

    // println!("general solution: x = {} + {} * t", t0, t1);

    // we then put that back into the original equation x1 * x + x0

    (x1 * t1, x1 * t0 + x0)
}

/// Returns the euclid "combinators" (a', b' such that a × a' + b × b' = gcd(a, b))
fn euclid_combinators(a: i64, b: i64) -> (i64, i64) {
    let div = a / b;
    let rem = a % b;

    if rem == 0 {
        (0, 1)
    } else {
        let (b_comb, rem_comb) = euclid_combinators(b, rem);
        (rem_comb, b_comb + rem_comb * -div)
    }
}
