use std::collections::HashSet;

use crdt_sample::{Aworset, NodeId};

fn get_a1_id() -> NodeId {
    NodeId::new(1, "a".to_string())
}

fn get_a2_id() -> NodeId{
    NodeId::new(2, "a".to_string())
} 

fn setup_a1() -> Aworset<String>{
    Aworset::new(get_a1_id())
}

fn setup_a2() -> Aworset<String>{
    Aworset::new(get_a2_id())
}


// ADD TESTS 

#[test]
fn add_letter(){

    let mut a1 = setup_a1();
    a1.add("A".to_string());

    let mut res = setup_a1();
    res.cc.insert((get_a1_id(), 1));
    res.set.insert((get_a1_id(), "A".to_string(), 1));
    res.local_counter = 2;

    assert_eq!(res, a1);
}

/// Adds empty string
#[test]
fn add_empty(){
    let mut a1 = setup_a1();
    a1.add("A".to_string());

    let mut res = setup_a1();
    res.cc.insert((get_a1_id(), 1));
    res.set.insert((get_a1_id(), "A".to_string(), 1));
    res.local_counter = 2;

    assert_eq!(res, a1);
}

#[test]
fn add_twice(){
    let mut a1 = setup_a1();
    a1.add("A".to_string());
    a1.add("A".to_string());

    let mut res = setup_a1();
    res.cc.insert((get_a1_id(), 1));
    res.cc.insert((get_a1_id(), 2));
    res.set.insert((get_a1_id(), "A".to_string(), 1));
    res.set.insert((get_a1_id(), "A".to_string(), 2));
    res.local_counter = 3;

    assert_eq!(res, a1);
}


// RM TESTS 

#[test]
fn rm_element(){
    // Given 
    let to_rm = "A".to_string();
    let mut a1 = setup_a1();
    a1.set.insert((get_a1_id(), "A".to_string(), 1));
    a1.cc.insert((get_a1_id(), 1));
    a1.local_counter = 2;

    let mut res = setup_a1();
    res.cc.insert((get_a1_id(), 1));
    res.local_counter = 2;

    // When 
    a1.rm(&to_rm);

    // Then
    assert_eq!(res, a1);
}

#[test]
fn rm_when_empty(){
    // Given 
    let to_rm = "A".to_string();
    let mut a1 = setup_a1();

    let mut res = setup_a1();
    res.local_counter = 1;

    // When 
    a1.rm(&to_rm);

    // Then
    assert_eq!(res, a1);
}

// JOIN TESTS 

#[test]
fn join_no_intersection(){
    // Given 
    let mut a1 = setup_a1();
    let mut a2 = setup_a2();

    a1.set.insert((get_a1_id(), "A".to_string(), 1));
    a1.cc.insert((get_a1_id(), 1));
    a1.local_counter += 1;
    a2.set.insert((get_a2_id(), "B".to_string(), 1));
    a2.cc.insert((get_a2_id(), 1));
    a2.local_counter += 1;

    let mut res = setup_a1();
    res.set.insert((get_a1_id(), "A".to_string(), 1));
    res.cc.insert((get_a1_id(), 1));
    res.set.insert((get_a2_id(), "B".to_string(), 1));
    res.cc.insert((get_a2_id(), 1));
    res.local_counter += 1;


    // When 
    a1.join(&a2);
    a2.join(&a1);

    // Then
    assert_eq!(res, a1);
    res.id = get_a2_id();
    assert_eq!(res, a2)
}


#[test]
fn join_intersection(){
    // Given 
    let mut a1 = setup_a1();
    let mut a2 = setup_a2();

    a1.set.insert((get_a1_id(), "A".to_string(), 1));
    a1.cc.insert((get_a1_id(), 1));
    a1.local_counter += 1;
    a2.set.insert((get_a1_id(), "A".to_string(), 1));
    a2.cc.insert((get_a1_id(), 1));
    a2.local_counter += 1;

    let mut res = setup_a1();
    res.set.insert((get_a1_id(), "A".to_string(), 1));
    res.cc.insert((get_a1_id(), 1));
    res.local_counter += 1;

    // When 
    a1.join(&a2);

    // Then
    assert_eq!(res, a1);
}

#[test]
fn elements(){
    let mut a1 = setup_a1(); 
    a1.set.insert((get_a1_id(), "A".to_string(), 1));
    a1.set.insert((get_a1_id(), "B".to_string(), 2));
    a1.set.insert((get_a2_id(), "B".to_string(), 1));
    a1.cc.insert((get_a1_id(), 1));
    a1.cc.insert((get_a1_id(), 2));
    a1.cc.insert((get_a2_id(), 1));

    let res = HashSet::from(["A".to_string(), "B".to_string()]);

    // When  
    let set = a1.elements();

    // Then 
    assert_eq!(set, res);
}