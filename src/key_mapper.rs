use std::hash::{Hash};
use std::collections::HashMap;
use ggez::event::{KeyCode, KeyMods};
use ggez::{Context};
use std::fmt;

use crate::GameState;

#[derive(Eq,PartialEq,Hash,Clone,Copy)]
pub enum KeyEdge{
    DOWN,
    UP,
    HELD,
}

impl fmt::Display for KeyEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UP => write!(f, "UP"),
            Self::DOWN => write!(f, "DOWN"),
            Self::HELD => write!(f, "HELD"),
        }
    }
}

#[derive(Eq,PartialEq,Hash,Clone,Copy)]
pub struct KeyEvent {
    key: KeyCode,
    modifier: KeyMods,
    edge: KeyEdge,
}

impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",format!("{:?} + {:?} + {}",self.modifier,self.key,self.edge).replace("NONE + ",""))
    }
}

#[derive(Clone,Copy)]
struct NamedFunction{
    name: &'static str,
    func: fn(&mut GameState),
}

pub struct KeyMapper {
    key_map: HashMap<KeyEvent, NamedFunction>,
    fun_vec: Vec<NamedFunction>,
    function_waiting: Option<NamedFunction>,
    ticks_held: usize,
    index: usize,
}

impl KeyMapper {
    pub fn new() -> KeyMapper {
        let key_map = HashMap::new();
        let fun_vec = Vec::new();
        KeyMapper {
            key_map,
            fun_vec,
            function_waiting: None,
            ticks_held: 0,
            index: 0,
        }
    }
    pub fn insert(&mut self, key: KeyCode, modifier: KeyMods, edge: KeyEdge, name: &'static str, func: fn(&mut GameState)){
        let ev = KeyEvent{
            key: key,
            modifier: modifier,
            edge: edge,
        }; 
        let named_func = NamedFunction{
            name: name,
            func: func as fn(&mut GameState),
        };
        let copy_func = named_func.clone();
        self.fun_vec.push(copy_func);
        //println!("{} {}", name, &ev);
        self.key_map.insert(ev,named_func);
    }
    pub fn remove_key_from_func(&mut self, name: &'static str)
    {
        let mut fun_key: Option<KeyEvent> = None;
        for (key,value) in &self.key_map{
            if value.name == name {
                fun_key = Some(*key);
            }
        }
        if let Some(value) = fun_key {
            self.key_map.remove(&value);
        }
    }
    pub fn add_key_to_func(&mut self, key: KeyEvent)
    {
        if let Some(func) = &self.function_waiting{
            if self.key_map.contains_key(&key) {
                self.key_map.remove(&key);
            }
            println!("Bound: {} to {}", func.name, key);
            self.key_map.insert(key, *func);
            self.function_waiting = None;
        }
    }
    pub fn set_waiting(&mut self, name: &'static str)
    {
        for named_func in &self.fun_vec{
            if named_func.name == name {
                self.function_waiting = Some(*named_func);
            }
        }
    }
    pub fn update(&mut self, m: &mut GameState, ctx: &Context){
        if self.function_waiting.is_none(){
            let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);
            let active_mods = ggez::input::keyboard::active_mods(ctx);
            for key in pressed_keys{
                self.event(m, *key, active_mods, KeyEdge::HELD);
            }
        }
        else {
            //println!("{}",self.ticks_held);
            if self.ticks_held > 0{
                self.ticks_held += 1;
            }
        }
    }
    pub fn event(&mut self, m: &mut GameState, key: KeyCode, modifier: KeyMods, edge: KeyEdge){
        // Create event struct
        let mut ev = KeyEvent{
            key: key,
            modifier: modifier,
            edge: edge,
        };
        // If there is some function waiting for a binding
        #[allow(unused_variables)]
        if let Some(func) = &self.function_waiting{
            // When a key is released set the type of event
            if edge==KeyEdge::UP {
                match self.ticks_held{
                    t if t > 59 => ev.edge = KeyEdge::UP,
                    t if t > 29 => ev.edge = KeyEdge::HELD,
                    t if t > 0 => ev.edge = KeyEdge::DOWN,
                    _ => (),
                }
                self.add_key_to_func(ev);
                self.ticks_held = 0;
            }
            // When a key is pressed start timer
            if edge==KeyEdge::DOWN{
                self.ticks_held = 1;
            }
        }
        // If there is no function waiting perform functions according to bindings
        else {
            match self.key_map.get(&ev){
                Some(mapping) => (mapping.func)(m),
                None => ()
            }
        }
    }
}

impl Iterator for KeyMapper{
    type Item = (String, &'static str);
    fn next(&mut self) -> Option<(String, &'static str)>{
        if self.index >= self.fun_vec.len(){
            self.index = 0;
            None
        }
        else
        {
            let fun_name = &self.fun_vec[self.index].name;
            let mut key_name = String::from("None");
            self.index += 1;
            // I can either iterate over the hashmap to see if a given function has a key bound to it
            // Or I can create another hashmap for the compliment of the key_map
            for (key,value) in &self.key_map{
                if value.name == *fun_name
                {
                    key_name = format!("{}",key);
                }
            }
            // If the function has a key mapped to it return it with the named function otherwise don't
            Some((key_name,fun_name))
        }
    }
}
