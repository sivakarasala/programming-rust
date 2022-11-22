fn print_padovan() {
    let mut padovan = vec![1,1,1]; // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}                                   // dropped here

fn main() {
    
    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    }                                       // both dropped here

    {
        struct Person { name: String, birth: i32 }

        let mut composers = Vec::new();
        composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
        composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
        composers.push(Person { name: "Lully".to_string(), birth: 1632 });

        for composer in &composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }

    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s;
        // let u = s; // error: use of moved value: `s`
        // let t = s.clone();
        // let u = s.clone();
        let u = t;
    }

    {
        let mut s = "Govinda".to_string();
        s = "Siddhartha".to_string(); // value "Govinda" dropped here
    }

    {
        let mut s = "Govinda".to_string();
        let t = s;
        s = "Siddhartha".to_string(); // nothing is dropped here
    }

    {
        let x = vec![10, 20, 30];
        let c = false;
        if c {
            //f(x); // ... ok to move from x here
        } else {
            //g(x); // ... and ok to also move from x here
        }
        // h(x); // bad: x is uninitialized here if either path uses it
    }

    {
        let x = vec![10, 20, 30];
        let mut y = 5;
        while y > 0 {
            // g(x); // bad: x would be moved in first iteration,
                    // uninitialized in second
            y -= 1;
        }
    }

    {
        let mut x = vec![10, 20, 30];
        let mut y = 5;
        while y > 0 {
            // g(x); // move from x
            // x = h(); // give x a fresh value
            y -= 1;
        }
    }

    // Moves and Indexed Content
    {
        // Build a vector of the strings "101", "102", ... "105"
        let mut v = Vec::new();
        for i in 101 .. 106 {
            v.push(i.to_string());
        }

        // Pull out random elements from the vector.
        // let third = v[2]; // error: Cannot move out of index of Vec
        // let fifth = v[4]; // here too
    }

    {
        let mut v = Vec::new();
        for i in 101 .. 106 {
            v.push(i.to_string());
        }

        // 1. Pop a value off the end of the vector:
        let fifth = v.pop().expect("vector empty!");
        assert_eq!(fifth, "105");

        // 2. Move a value out of a given index in the vector,
        // and move the last element into its spot:
        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        // 3. Swap in another value for the one we're taking out:
        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        // Let's see what's left of our vector.
        assert_eq!(v, vec!["101", "104", "substitute"]);
    }

    {
        let v = vec!["liberté".to_string(),
                    "égalité".to_string(),
                    "fraternité".to_string()];

        for mut s in v {
            s.push('!');
            println!("{}", s);
        }
    }

    {
        struct Person { name: Option<String>, birth: i32 }

        let mut composers = Vec::new();
        composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });

        // let first_name = composers[0].name; // not valid

        let first_name = std::mem::replace(&mut composers[0].name, None);
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }

    {
        struct Person { name: Option<String>, birth: i32 }

        let mut composers = Vec::new();
        composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });

        // let first_name = composers[0].name; // not valid

        let first_name = composers[0].name.take();
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }

    {
        use std::rc::Rc;

        // Rust can infer all these types; written out for clarity
        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let u: Rc<String> = s.clone();

        assert!(s.contains("shira"));
        assert_eq!(t.find("taki"), Some(5));
        println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    }
}
