#[derive(Debug)]
pub enum AstType {
  Root,
  Start,
  End,
  Normal,
}

#[derive(Debug)]
pub struct PartArena {
  pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
  id: usize,
  start: usize,
  end: usize,
  kind: AstType,
  value: char,
  children: Vec<Option<usize>>,
  parent: Option<usize>,
}

impl Part {
  fn new(kind: AstType, imput: char, start: usize) -> Part {
    Part {
      id: 0,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: None,
    }
  }

  fn add_child(&mut self, id: usize, kind: AstType, imput: char, start: usize) -> Part {
    let p = Part {
      id: id,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: Some(self.id),
    };
    self.children.push(Some(p.id));
    p
  }
}

#[derive(Debug)]
pub enum WalkingType {
  Function,
  Normal,
}

#[derive(Debug)]
struct Walker<'a> {
  input: &'a str,
  part: usize,
  part_arena: PartArena,
  current_type: WalkingType,
}

impl <'a>Walker<'a> {
  fn new(input: &str) -> Walker {
    let mut pa = PartArena{ parts: Vec::new() };
    pa.parts.push(Part::new(
      AstType::Root, ' ', 0
    ));
    
    Walker {
      input: input,
      part: 0,
      part_arena: pa,
      current_type: WalkingType::Function,
    }
  }

  pub fn add_child_to_part(&mut self, part_index: usize, kind: AstType, imput: char, start: usize) -> Part {
    let length = self.part_arena.parts.len();
    let part = self.part_arena.parts.get_mut(part_index).unwrap();
    part.add_child(length + 1, kind, imput, start)
  }

  pub fn walk(&mut self) {
    let mut chars = self.input.chars();
    let mut index: usize = 0;
    let mut part_id: usize = 0;
    let mut new_part: Part;

    while let Some(cha) = chars.next() {
      {
        let mut part = &mut self.part_arena.parts.get_mut(part_id).unwrap();
        new_part = match cha {
          '{' => {
            let child_part = part.add_child(index, AstType::Start, cha, index);
            part_id = child_part.id;
            child_part
          },
          '}' => {
            part_id = part.id;
            let child_part = part.add_child(index, AstType::Start, cha, index);
            child_part
          },
          ' ' => {
            index += 1;
            continue
          },
          _ => {
            let child_part = part.add_child(index, AstType::Start, cha, index);
            part_id = child_part.id;
            child_part
          } 
        };
      }
      self.part_arena.parts.push(new_part);
      index += 1;
    }
  }
}

fn main() {
  let mut walker = Walker::new("{a b  c}");
  walker.walk();

  let nyn: String = "abc".to_string();
  println!("{}", nyn);
}

// impl fmt::Debug for Part {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }
