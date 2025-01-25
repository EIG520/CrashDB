import logo from './logo.svg';
import './App.css';
import { useState } from 'react';


class TreeNode {
    constructor(label, children) {
        this.display = label;
        this.children = children;
    }
}

const el_banana1 = new TreeNode("banana1", []);
const el_banana2 = new TreeNode("banana2", []);
const el_banana3 = new TreeNode("banana3", []);
const el_bananas = new TreeNode("bananas", [el_banana1, el_banana2, el_banana3]);
const el_apple1 = new TreeNode("apple1", []);
const el_apple2 = new TreeNode("apple2", []);
const el_apples = new TreeNode("apples", [el_apple1, el_apple2]);
const el_foods = new TreeNode("foods", [el_apples, el_bananas]);
const el_root = new TreeNode("root", [el_foods]);

function Entry({ label, children }) {
  const [open, setopen] = useState(false);
  return (<div>
    <button onClick = {() => setopen(!open)}>
      {label}
    </button>
    <ol>{open ? children : ""}</ol>
  </div>);
}

function toTree(root) {
    return (
      <li><Entry label={root.display} children={root.children.map((entry) => toTree(entry))}/></li>
    );
}

function App() {
  return (
    <ol>{toTree(el_root)}</ol>
  );
}

export default App;
