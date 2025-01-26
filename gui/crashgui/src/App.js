import logo from './logo.svg';
import './App.css';
import './com';
import { useState, useRef, useReducer, useContext, createContext } from 'react';
import "./App.css";

class TreeNode {
    constructor(label, children, open, open_ready, path) {
        this.display = label;
        this.children = children;
        this.open = open;
        this.open_ready = open_ready;
        this.selected = false;
        this.path = path;
    }
}

const el_root = new TreeNode("node", [], false, false, []);

function InfoBar() {
  const forceupdate = useContext(GeneralContext).forceupdate;
  const eref = useContext(GeneralContext).inforef;
  const data = useContext(GeneralContext).data;

  return (
    <div className="sidebar">
      <div className="center">
        <h1>{eref.current && eref.current.children[0].innerHTML}</h1>
        {data && (data.open 
          ? <button onClick = {() => {data.open=false;forceupdate()}}>Close</button>
          : <button onClick = {() => {data.open=true;forceupdate()}}>Open</button>
        )}
      </div>
    </div>
  )
}

function Entry({ children, root }) {
  const ref = useRef(null);
  const setinforef = useContext(GeneralContext).setinforef;
  const setdata = useContext(GeneralContext).setdata;
  const data = useContext(GeneralContext).data;
  const forceupdate = useContext(GeneralContext).forceupdate;

  if (root.open && !root.open_ready) {
    const new_path = [...root.path, root.display];

    root.children[0] = (new TreeNode(root.display + ".0", [], false, false, new_path));
    root.children[1] = (new TreeNode(root.display + ".1", [], false, false, new_path));
    root.children[2] = (new TreeNode(root.display + ".2", [], false, false, new_path));
    root.open_ready = true;

    forceupdate();
  }

  return (<div ref={ref}>
    <button className={root.selected ? "selected" : "entry"} onClick = {() => {root.selected=true;data.selected=false;setinforef(ref);setdata(root)}}>
      {root.display}
    </button>
    <ul>{root.open && children}</ul>
  </div>);
}

function toTree(root) {
  return (
    <li key={root.display}><Entry root={root} children={root.children.map((entry) => toTree(entry))}/></li>
  );
}

const GeneralContext = createContext({});

function App() {
  const [inforef, setinforef] = useState(useRef(null));
  const [data, setdata] = useState(null);
  const [_, forceupdate] = useReducer((x) => x+1, 0);

  return (
    <GeneralContext.Provider value={{setinforef:setinforef, setdata:setdata, forceupdate:forceupdate, inforef: inforef, data:data}}>
      <InfoBar/>
      <ul>{toTree(el_root)}</ul>
    </GeneralContext.Provider>
  );
}

export default App;
