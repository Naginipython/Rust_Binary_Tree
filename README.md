# Rust_Binary_Tree

This was a personal test to see if I can create a Binary Search Tree in Rust. 

### Binaries

#### BST w/Nodes

The final product is inside src/bin/node_based.rs. This is the primary reason I created this project, because attempting to recursively go through structs that contain values such as Box, Rc, Arc, has proven to be a small challenge for me as of late. In this project, I have BST struct, representing a Binary Search Tree. It's only field is a "root" field, where another struct, called a Node, is contained. The Node simply has an i32 data field, and a left/right. The BST goes through every node and places it appropriately.
Update: With more inspiration from the reference, I have made the entirety of this generic. 

#### BST

Another implementation of a BST. This was a test to see if I can create a BST using the BST itself as the node. The BST contained the fields: data, left, and right. The BST simply traverses through itself and places a new BST with the data to the left/right of where it is appropriate.

#### Reference/motivator

When I initally failed to create the BST a day prior, I searched the internet for other implementations. I found that a Median article explained their process, and instead of reading, I downloaded their code and checked out their implementation. Afterwards, I was determined to do it my way, firstly, creating a similar version to this and my own idea, with the above BST. 