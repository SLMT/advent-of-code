///
/// I have to admit that my solution is way more complicated than the solution 
/// that we need to solve this problem. I originally assumed that the number of 
/// 'X' bits shown in the masks from inputs were much more, so I implemented a 
/// compact trie, which I redesign to make it faster and smaller. However, after
///  I checked other solutions written by other people, I found that this
/// problem can be easily solved by brute-force all possible addresses.
///
/// If you are looking for a quick solution for this problem, the following is
/// definately not what you want.
///
/// by SLMT (27th Dec 2020)
///

use std::io;

// For debug
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct MaskedBits {
  bits: Vec<Option<bool>>
}

impl Display for MaskedBits {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut string = String::new();
    for bit in &self.bits {
      match bit {
        Some(true) => string.push('1'),
        Some(false) => string.push('0'),
        None => string.push('X'),
      }
    }
    write!(f, "{}", string)
  }
}

impl MaskedBits {
  fn from_str(string: &str) -> MaskedBits {
    let mut mask = Self::new();

    for (idx, ch) in string.chars().enumerate() {
      mask.bits[idx] = match ch {
        '0' => Some(false),
        '1' => Some(true),
        'X' => None,
        _ => panic!("Unknown character: {}", ch)
      };
    }

    mask
  }

  fn new() -> MaskedBits {
    MaskedBits {
      bits: vec![None; 36]
    }
  }

  // "Full" means 36-bit masking
  fn full_mask(&self, input: i64) -> MaskedBits {
    // Change the input to binary string
    let old_bits = MaskedBits::from_str(&format!("{:036b}", input));

    // Mask the bits
    let mut new_bits = vec![];
    for (old_bit, mask) in old_bits.bits.iter().zip(self.bits.iter()) {
      match mask {
        &Some(true) => new_bits.push(Some(true)),
        &Some(false) => new_bits.push(old_bit.clone()),
        &None => new_bits.push(None),
      }
    }

    // Output as a MaskedBits object
    MaskedBits {
      bits: new_bits
    }
  }
}

#[derive(Debug)]
enum Instruction {
  SetMask(MaskedBits),
  SetValue(usize, i64) // (address, value)
}

impl Instruction {
  fn from_str(line: &str) -> Instruction {
    match &line[0..3] {
      "mas" => {
        let mask = line.split("=").nth(1).unwrap().trim();
        Instruction::SetMask(MaskedBits::from_str(mask))
      },
      "mem" => {
        let mut tokens = line.split("=");
        let address: usize = tokens.next().unwrap()
          .split("[").nth(1).unwrap()
          .split("]").next().unwrap()
          .trim().parse().unwrap();
        let value: i64 = tokens.next().unwrap()
          .trim().parse().unwrap();
        Instruction::SetValue(address, value)
      },
      _ => panic!("Unknown instruction: {}", line)
    }
  }
}

#[derive(Clone)]
struct MemoryTrieBranchNode {
  zero_side: Option<Box<MemoryTrieNode>>,
  one_side: Option<Box<MemoryTrieNode>>,
}

impl Display for MemoryTrieBranchNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[0: {}, 1: {}]",
      self.zero_side.as_ref().unwrap(),
      self.one_side.as_ref().unwrap()
    )
  }
}

#[derive(Clone)]
struct MemoryTrieHighwayNode {
  highway_bits: MaskedBits,
  next_node: Option<Box<MemoryTrieNode>>
}

impl Display for MemoryTrieHighwayNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[\"{}\": {}]",
      self.highway_bits,
      self.next_node.as_ref().unwrap()
    )
  }
}

impl MemoryTrieHighwayNode {
  fn new_with_data(address: MaskedBits, data: i64) -> MemoryTrieNode {
    let highway = MemoryTrieHighwayNode {
      highway_bits: address.clone(),
      next_node: Some(Box::new(MemoryTrieNode::Data(data)))
    };
    MemoryTrieNode::Highway(highway)
  }

  // Split [Highway Node A] -> [Node B]
  // into [Highway Node A] -> [Branch Node C] -> [Highway Node D] -> [Node B1]
  //                                          -> [Highway Node E] -> [Node B2]
  fn branch_by_copy(self, branch_point: usize) -> MemoryTrieNode {
    // Quick Check
    if self.highway_bits.bits[branch_point] != None {
      panic!("Cannot copy split at non-'X' point");
    }

    // Create a branch
    self.create_branch(branch_point, None, None)
  }

  // Split [Highway Node A] -> [Node B]
  // into [Highway Node A] -> [Branch Node C] 0-> [Highway Node D] -> [New Node]
  //                                          1-> [Highway Node E] -> [Node B]
  fn branch_by_new_node(
    self,
    branch_point: usize,
    new_node: MemoryTrieNode
  ) -> MemoryTrieNode {

    // Decide the nodes after the branch node
    let (zero, one) = match self.highway_bits.bits[branch_point] {
      Some(true) => (Some(Box::new(new_node)), None),
      Some(false) => (None, Some(Box::new(new_node))),
      None => panic!("Cannot add branch at 'X' point")
    };

    // Create a branch
    self.create_branch(branch_point, zero, one)
  }

  fn create_branch(
    mut self,
    branch_point: usize,
    zero_side_node: Option<Box<MemoryTrieNode>>,
    one_side_node: Option<Box<MemoryTrieNode>>,
  ) -> MemoryTrieNode {
    // Create a node after the branch node
    let after_node = if branch_point == self.highway_bits.bits.len() - 1 {
      self.next_node.take().unwrap()
    } else {
      let last_seq = self.highway_bits.bits[(branch_point + 1)..].to_vec();
      let new_highway = MemoryTrieHighwayNode {
        highway_bits: MaskedBits { bits: last_seq },
        next_node: self.next_node.take()
      };
      Box::new(MemoryTrieNode::Highway(new_highway))
    };

    // Create a branch node
    // and put the given nodes at each branch.
    // If there is no gvien node, put the after node.
    let branch = MemoryTrieNode::Branch(MemoryTrieBranchNode {
      zero_side: Some(zero_side_node.unwrap_or(after_node.clone())),
      one_side: Some(one_side_node.unwrap_or(after_node.clone())),
    });

    // Update or delete the current highway
    if branch_point == 0 {
      branch
    } else {
      let first_seg = self.highway_bits.bits[0..branch_point].to_vec();
      self.highway_bits = MaskedBits { bits: first_seg };
      self.next_node.replace(Box::new(branch));
      MemoryTrieNode::Highway(self)
    }
  }
}

#[derive(Clone)]
enum MemoryTrieNode {
  Branch(MemoryTrieBranchNode),
  Highway(MemoryTrieHighwayNode),
  Data(i64)
}

impl Display for MemoryTrieNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      MemoryTrieNode::Branch(branch) => write!(f, "Branch: {}", branch),
      MemoryTrieNode::Highway(highway) => write!(f, "Highway: {}", highway),
      MemoryTrieNode::Data(value) => write!(f, "Data: ({})", value)
    }
  }
}

// Implement the memory structure using a bitwise Trie
struct MemoryTrie {
  root: Option<Box<MemoryTrieNode>>
}

impl Display for MemoryTrie {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self.root {
      Some(root) => write!(f, "{}", root),
      None => write!(f, "Empty")
    }
  }
}

impl MemoryTrie {
  fn new() -> MemoryTrie {
    MemoryTrie {
      root: None
    }
  }
  
  fn update_node(
    node: Box<MemoryTrieNode>,
    address_bits: &[Option<bool>],
    value: i64
  ) -> Box<MemoryTrieNode> {
    match *node {
      MemoryTrieNode::Branch(mut branch) => {
        if address_bits[0] != Some(false) {
          let one_side = Self::update_node(
            branch.one_side.take().unwrap(),
            &address_bits[1..],
            value
          );
          branch.one_side.replace(one_side);
        }
        if address_bits[0] != Some(true) {
          let zero_side = Self::update_node(
            branch.zero_side.take().unwrap(),
            &address_bits[1..],
            value
          );
          branch.zero_side.replace(zero_side);
        }

        Box::new(MemoryTrieNode::Branch(branch))
      },
      MemoryTrieNode::Highway(mut highway) => {
        // Compare each bit
        let bit_count = highway.highway_bits.bits.len();
        for idx in 0 .. bit_count {
          let highway_bit = highway.highway_bits.bits[idx];
          if highway_bit != address_bits[idx] {
            // X case
            if highway_bit == None {
              // Create a branch (by copying the sub-tree)
              let highway = highway.branch_by_copy(idx);

              // Lookup again
              return Self::update_node(
                Box::new(highway),
                address_bits,
                value
              );
            
            // 0 or 1 case (need a new branch)
            } else {
              // Create a new node for the following bits
              let later_bits = address_bits[(idx + 1)..].to_vec();
              let new_node = MemoryTrieHighwayNode::new_with_data(
                MaskedBits { bits: later_bits },
                value
              );

              // Add a branch (by adding a new node)
              let highway = highway.branch_by_new_node(idx, new_node);

              // A special case: the lookup bit is 'X' => must handle other side
              if address_bits[idx] == None {
                let mut new_address = address_bits.to_vec();
                new_address[idx] = Some(highway_bit.unwrap());
                return Self::update_node(
                  Box::new(highway),
                  &new_address,
                  value
                );
              } else {
                return Box::new(highway);
              }
            }
          }
        }

        // All bits are the same
        let next_node = Self::update_node(
          highway.next_node.take().unwrap(),
          &address_bits[bit_count..],
          value
        );
        highway.next_node.replace(next_node);

        Box::new(MemoryTrieNode::Highway(highway))
      },
      MemoryTrieNode::Data(_) => {
        Box::new(MemoryTrieNode::Data(value))
      }
    }
  }

  fn dfs_sum(node: &MemoryTrieNode) -> i64 {
    match node {
      MemoryTrieNode::Branch(branch) => {
        Self::dfs_sum(&branch.zero_side.as_ref().unwrap()) +
          Self::dfs_sum(&branch.one_side.as_ref().unwrap())
      },
      MemoryTrieNode::Highway(highway) => {
        let combination_count = highway.highway_bits.bits.iter()
          .filter(|bit| **bit == None)
          .count();
        let value = Self::dfs_sum(&highway.next_node.as_ref().unwrap());
        2i64.saturating_pow(combination_count as u32) * value
      },
      MemoryTrieNode::Data(value) => *value
    }
  }

  fn write(&mut self, address: MaskedBits, value: i64) {
    let new_root = if let Some(root) = self.root.take() {
      Self::update_node(
        root,
        &address.bits,
        value
      )
    } else {
      // Create a (highway -> data) route
      Box::new(MemoryTrieHighwayNode::new_with_data(address.clone(), value))
    };
    self.root.replace(new_root);
  }

  fn sum(&self) -> i64 {
    if let Some(root) = self.root.as_ref() {
      Self::dfs_sum(root)
    } else {
      0
    }
  }
}


fn main() {
  let mut input = String::new();
  let mut lines = vec![];

  // Read input as lines
  loop {
    input.clear();
    let read_count = io::stdin().read_line(&mut input).unwrap();
    if read_count == 0 { // EOF
      break;
    }
    lines.push(input.trim().to_owned());
  }

  println!("{}", solve(lines));
}

fn solve(lines: Vec<String>) -> i64 {
  let instructions: Vec<Instruction> = lines.into_iter()
    .map(|line| Instruction::from_str(&line))
    .collect();

  let mut memory = MemoryTrie::new();
  let mut mask = MaskedBits::new();
  for ins in instructions {
    execute(&ins, &mut mask, &mut memory);

    // For debug
    // println!("{}", &memory);
  }

  memory.sum()
}

fn execute(
  ins: &Instruction,
  mask: &mut MaskedBits,
  memory: &mut MemoryTrie
) {
  match ins {
    Instruction::SetMask(new_mask) => *mask = new_mask.clone(),
    Instruction::SetValue(addr, value) => {
      let address = mask.full_mask(*addr as i64);
      memory.write(address, *value);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let lines: Vec<String> = vec![
      "mask = 000000000000000000000000000000X1001X",
      "mem[42] = 100",
      "mask = 00000000000000000000000000000000X0XX",
      "mem[26] = 1",
    ].into_iter().map(|s| s.to_owned()).collect();
    assert_eq!(solve(lines), 208);
  }
}
