s/\[/Val::List(vec![/g
s/]/])/g
s/\([0-9][0-9]*\)/Val::Number(\1)/g

1~3 s/^/pairs.push(Pair::new(/
1~3 s/$/,/
2~3 s/$/));/
