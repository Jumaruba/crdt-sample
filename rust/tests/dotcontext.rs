use crdt_sample::DotContext;


#[test]
fn create(){
    // Must start empty.
    let dot_ctx : DotContext<String> = DotContext::new();

    println!("{:?}", dot_ctx);
}


/// Tests insert dot function without compacting, because this function
/// should not depend on compacting. 
#[test]
fn insert_dot(){
    // Given
    let mut dot_ctx : DotContext<String> = DotContext::new();
    let dot = ("A".to_string(), 3);
    let mut res : DotContext<String> = DotContext::new();
    res.dc.insert(dot.clone());

    // When 
    dot_ctx.insert_dot(&dot, Some(false));

    // Then
    assert_eq!(dot_ctx, res);
    println!("{:?}", dot_ctx);
}


#[test]
fn compact_not_possible(){
    // Given
    let dot = ("A".to_string(), 3);
    let mut dot_ctx : DotContext<String> = DotContext::new();
    dot_ctx.dc.insert(dot.clone());
    let mut res : DotContext<String> = DotContext::new();
    res.dc.insert(dot.clone());

    // When 
    dot_ctx.compact();

    // Then
    assert_eq!(dot_ctx, res);
    println!("{:?}", dot_ctx);
}

#[test]
fn compact_possible_1(){
    // Given
    let mut dot_ctx : DotContext<String> = DotContext::new();
    dot_ctx.dc.insert(("A".to_string(),1));
    dot_ctx.dc.insert(("A".to_string(),2));
    dot_ctx.dc.insert(("A".to_string(),3));
    let mut res : DotContext<String> = DotContext::new();
    res.cc.insert("A".to_string(), 3);

    // When 
    dot_ctx.compact();

    // Then
    assert_eq!(dot_ctx, res);
    println!("{:?}", dot_ctx);
}

#[test]
fn compact_possible_and_not(){
    // Given
    let mut dot_ctx : DotContext<String> = DotContext::new();
    dot_ctx.dc.insert(("A".to_string(),1));
    dot_ctx.dc.insert(("A".to_string(),2));
    dot_ctx.dc.insert(("A".to_string(),3));
    dot_ctx.dc.insert(("A".to_string(),5));
    let mut res : DotContext<String> = DotContext::new();
    res.cc.insert("A".to_string(), 3);
    res.dc.insert(("A".to_string(), 5));

    // When 
    dot_ctx.compact();

    // Then
    assert_eq!(dot_ctx, res);
    println!("{:?}", dot_ctx);
}

/// This function depends on compact.
#[test]
fn join(){
    // {dc: {(A,3)}; cc: A->1}
    let mut dot_ctx_1 : DotContext<String> = DotContext::new();
    dot_ctx_1.cc.insert("A".to_string(), 1);
    dot_ctx_1.dc.insert(("A".to_string(), 3));
    // {dc: {}, cc: A-> 4}
    let mut dot_ctx_2 : DotContext<String> = DotContext::new();
    dot_ctx_2.cc.insert("A".to_string(), 4);

    let mut res: DotContext<String> = DotContext::new();
    res.cc.insert("A".to_string(), 4);

    // When 
    dot_ctx_1.join(&dot_ctx_2);

    // Then 
    assert_eq!(res, dot_ctx_1);
    println!("{:?}", dot_ctx_1);
}