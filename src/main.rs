fn main() {
    let mut my_comp = Component {
        state: true,
        count: 0,
    };

    let mut my_comp2 = Component {
        state: false,
        count: 1,
    };
    my_comp.get_state();
    my_comp.switch_state();
    my_comp.get_state();

    let linked_components = ComponentLink {
        component_a: my_comp,
        component_b: my_comp2,
    };

    linked_components.get_states();
}

struct ComponentLink {
    component_a: Component,
    component_b: Component,
}

impl ComponentLink {
    fn get_states(&self) {
        println!("{} {}", self.component_a.state, self.component_b.state);
    }
}


struct Component {
    state: bool,
    count: u32
}

impl Component {
    fn switch_state(&mut self) {
        self.state = !self.state;
    }
    
    fn get_state(&self) {
        println!("{}", self.state);
    }
}
