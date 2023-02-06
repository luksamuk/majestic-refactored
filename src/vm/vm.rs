use super::types::*;

impl VirtualMachine {
    pub fn new() -> Box<Self> {
        let mut vm = Box::new(VirtualMachine::default());

        // The order of these symbols must match the ConstSymbol enum!
        let primitive_atoms = vec![
            "nil",
            "t",
            "prim",
            "lit",
            "closure",
            "error",
            "fn",
            "&",
            "apply",
            "macro",
            "mac",
            "quote",
            "unquote",
            "unquote-splice",
            "quasiquote",
            "do",
            "integer",
            "float",
            "fraction",
            "complex",
            "vector",
            "setq",
        ];

        let self_evaluating_atoms = vec!["nil", "t"];

        let make_self_evaluating = |vm: &mut Self, name| {
            let atom = vm.make_atom(name).unwrap();
            let _ = vm.assign_value(atom.clone(), atom);
        };

        for atom_name in primitive_atoms {
            let _ = vm.make_atom(atom_name);
        }

        for atom_name in self_evaluating_atoms {
            make_self_evaluating(&mut vm, atom_name);
        }

        // TODO: Add primitive functions

        vm
    }

    fn format_bytes(mut num: usize) -> String {
        let units = vec!["bytes", "KB", "MB", "GB"];
        let mut unit = 0;

        for i in 1..units.len() {
            if num >= 1024 {
                num = num / 1024;
            } else {
                unit = i - 1;
                break;
            }
        }

        format!("{} {}", num, units[unit])
    }

    pub fn debrief(&self) {
        use comfy_table::modifiers::UTF8_ROUND_CORNERS;
        use comfy_table::presets::UTF8_FULL_CONDENSED;
        use comfy_table::*;

        let used_atom_table = self.atoms.last * std::mem::size_of::<Atom>();
        let used_number_table = self.atoms.last * std::mem::size_of::<Number>();
        let used_list_area = self.lists.last * std::mem::size_of::<ListArea>();
        let used_stack_area = self.stack.last * std::mem::size_of::<StackArea>();

        let atom_table_size = ATOM_TABLE_SIZE * std::mem::size_of::<Atom>();
        let number_table_size = NUMBER_TABLE_SIZE * std::mem::size_of::<Number>();
        let list_area_size = LIST_AREA_SIZE * std::mem::size_of::<Cons>();
        let stack_area_size = LISP_STACK_SIZE * std::mem::size_of::<TypedPointer>();

        let total_size = atom_table_size + number_table_size + list_area_size + stack_area_size;

        println!("VM Statistics");

        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.apply_modifier(UTF8_ROUND_CORNERS);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(&vec!["Statistics", "Current", "Total", "Unit", "Contents"]);

        table.add_row(vec![
            "Atom Table",
            &Self::format_bytes(used_atom_table),
            &Self::format_bytes(atom_table_size),
            &Self::format_bytes(std::mem::size_of::<Atom>()),
            &format!("{} atoms", self.atoms.last),
        ]);

        table.add_row(vec![
            "Number Table",
            &Self::format_bytes(used_number_table),
            &Self::format_bytes(number_table_size),
            &Self::format_bytes(std::mem::size_of::<Number>()),
            &format!("{} numbers", self.numbers.last),
        ]);

        table.add_row(vec![
            "List Area",
            &Self::format_bytes(used_list_area),
            &Self::format_bytes(list_area_size),
            &Self::format_bytes(std::mem::size_of::<Cons>()),
            &format!("{} cells", self.lists.last),
        ]);

        table.add_row(vec![
            "Stack Area",
            &Self::format_bytes(used_stack_area),
            &Self::format_bytes(stack_area_size),
            &Self::format_bytes(std::mem::size_of::<TypedPointer>()),
            &format!("{} pointers", self.stack.last),
        ]);

        table.add_row(vec![
            "Total Size",
            &Self::format_bytes(
                used_atom_table + used_number_table + used_list_area + used_stack_area,
            ),
            &Self::format_bytes(total_size),
            "-",
            "-",
        ]);

        println!("{}", table);
    }

    pub fn print_list_area(&self) {
        use comfy_table::modifiers::UTF8_ROUND_CORNERS;
        use comfy_table::presets::UTF8_BORDERS_ONLY;
        use comfy_table::*;

        let mut table = Table::new();
        table.load_preset(UTF8_BORDERS_ONLY);
        table.apply_modifier(UTF8_ROUND_CORNERS);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(&vec!["ADDR", "CAR", "CDR"]);

        for i in 0..self.lists.last {
            let cons = self.lists.area.get(i).unwrap();
            table.add_row(vec![
                &format!("{:#08x}", i),
                &format!("{}", cons.car),
                &format!("{}", cons.cdr),
            ]);
        }

        println!("{}", table);
    }

    pub fn print_atom_table(&self) {
        use comfy_table::modifiers::UTF8_ROUND_CORNERS;
        use comfy_table::presets::UTF8_BORDERS_ONLY;
        use comfy_table::*;

        let mut table = Table::new();
        table.load_preset(UTF8_BORDERS_ONLY);
        table.apply_modifier(UTF8_ROUND_CORNERS);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(&vec!["ADDR", "NAME", "VALUE"]);

        for i in 0..self.atoms.last {
            let atom = self.atoms.area.get(i).unwrap();
            table.add_row(vec![
                &format!("{:#08x}", i),
                &atom.name,
                &format!("{}", atom.value),
            ]);
        }

        println!("{}", table);
    }

    pub fn print_number_table(&self) {
        use comfy_table::modifiers::UTF8_ROUND_CORNERS;
        use comfy_table::presets::UTF8_BORDERS_ONLY;
        use comfy_table::*;

        let mut table = Table::new();
        table.load_preset(UTF8_BORDERS_ONLY);
        table.apply_modifier(UTF8_ROUND_CORNERS);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(&vec!["ADDR", "VALUE"]);

        for i in 0..self.numbers.last {
            let num = self.numbers.area.get(i).unwrap();
            table.add_row(vec![&format!("{:#08x}", i), &format!("{}", num)]);
        }

        println!("{}", table);
    }
}
