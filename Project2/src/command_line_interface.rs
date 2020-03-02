use crate::red_black_tree::RedBlackTree;
use crate::avl_tree::AVLTree;

use std::str::FromStr;
use std::io::{stdin, stdout, Write};

pub struct CommandLineInterface{}

impl CommandLineInterface{
    pub fn run(&self){
        let mut intro = 1;

        // used to test the library
        loop {
            if intro == 1 {
                // ask user to test a tree type
                println!("\n\nType \"rbt\" or \"avl\" to select the type of tree to test.");
                println!("Type help at any time for a list of available commands.\n");
                intro = 0;
            }
            
            // print the users cursor
            print!("Select Tree > ");
            stdout().flush().unwrap();

            // get the users input
            let mut input = String::new();
            let line = stdin().read_line(&mut input);

            match line{
                Ok(val) => {
                    if val < 3{ // if input is null
                        continue;
                    }
                }
                _ => {},
            }

            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();

            // match the command to an action
            match command {
                "exit" => return,
                "rbt" => {
                    self.run_rbt();
                },
                "avl" => {
                    self.run_avl();
                },
                "help" => {
                    println!("\nAvailable commands:\n");
                    println!("    exit - Exit the program.");
                    println!("    rbt  - Test the red black tree library.");
                    println!("    avl  - Test the avl tree library.");
                    println!("    help - Show available commands.");
                    println!();
                },
                command => {
                    println!("Command {:?} not recognized. Type help for available commands.", command);
                }
            }
        }
    }

    fn run_rbt(&self) {
        let mut rbtree = RedBlackTree::<u32>::new();
        loop {
            // print the users cursor
            print!("Testing RBT > ");
            stdout().flush().unwrap();
    
            // get the users input
            let mut input = String::new();
            let line = stdin().read_line(&mut input);

            match line{
                Ok(val) => {
                    if val < 3{ // if input is null
                        continue;
                    }
                }
                _ => {},
            }

            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
    
            // match the command to an action
            match command {
                "exit" => return,
                "help" => {
                    println!("\nAvailable commands:\n");
                    println!("    exit       - Exit the program. (will delete the current rbt)");
                    println!("    help       - Shows available commands.");
                    println!("    insert VAL - Inserts \"VAL\" into the tree.");
                    println!("    delete VAL - Deletes \"VAL\" from the tree.");
                    println!("    count      - Returns the number of leaves in the tree.");
                    println!("    height     - Returns the height of the tree.");
                    println!("    print      - Prints out the node information in order.");
                    println!("    isempty    - Returns 0 if the tree is empty, otherwise 1.");
                    println!();
                },
                "insert" => {
                    match args.peekable().peek() {
                        Some(val) => {
                            rbtree.insert(FromStr::from_str(val).unwrap());
                        },
                        None => {
                            println!("Must add a value after \"insert\"");
                        }
                    }
                },
                "delete" => {
                    match args.peekable().peek() {
                        Some(val) => {
                            rbtree.delete(FromStr::from_str(val).unwrap());
                        },
                        None => {
                            println!("Must add a value after \"delete\"");
                        }
                    }
                },
                "count" => {
                    println!("{:?}", rbtree.count_leaf_nodes());
                },
                "height" => {
                    println!("{:?}", rbtree.get_height());
                },
                "print" => {
                    rbtree.print();
                },
                "isempty" => {
                    println!("{:?}", rbtree.is_empty());
                },
                command => {
                    println!("Command {:?} not recognized. Type help for available commands.", command);
                }
            }
        }
    }

    fn run_avl(&self) {
        let mut avltree = AVLTree::<u32>::new();
        loop {
            // print the users cursor
            print!("Testing AVL > ");
            stdout().flush().unwrap();
    
            // get the users input
            let mut input = String::new();
            let line = stdin().read_line(&mut input);

            match line{
                Ok(val) => {
                    if val < 3{ // if input is null
                        continue;
                    }
                }
                _ => {},
            }

            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
    
            // match the command to an action
            match command {
                "exit" => return,
                "help" => {
                    println!("\nAvailable commands:\n");
                    println!("    exit       - Exit the program. (will delete the current avl tree)");
                    println!("    help       - Shows available commands.");
                    println!("    insert VAL - Inserts \"VAL\" into the tree.");
                    println!("    delete VAL - Deletes \"VAL\" from the tree.");
                    println!("    count      - Returns the number of leaves in the tree.");
                    println!("    height     - Returns the height of the tree.");
                    println!("    print      - Prints out the node information in order.");
                    println!("    isempty    - Returns 0 if the tree is empty, otherwise 1.");
                    println!();
                },
                "insert" => {
                    match args.peekable().peek() {
                        Some(val) => {
                            avltree.insert(FromStr::from_str(val).unwrap());
                        },
                        None => {
                            println!("Must add a value after \"insert\"");
                        }
                    }
                },
                "delete" => {
                    match args.peekable().peek() {
                        Some(val) => {
                            avltree.delete(FromStr::from_str(val).unwrap());
                        },
                        None => {
                            println!("Must add a value after \"delete\"");
                        }
                    }
                },
                "count" => {
                    println!("{:?}", avltree.count_leaf_nodes());
                },
                "height" => {
                    println!("{:?}", avltree.get_height());
                },
                "print" => {
                    println!("{:?}", avltree.print_in_order_traversal());
                },
                "isempty" => {
                    println!("{:?}", avltree.is_empty());
                },
                command => {
                    println!("Command {:?} not recognized. Type help for available commands.", command);
                },
            }
        }
    }

}
