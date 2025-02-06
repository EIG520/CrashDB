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

const el_root = new TreeNode("thing", [], false, false, []);

function InfoBar() {
  const forceupdate = useContext(GeneralContext).forceupdate;
  const eref = useContext(GeneralContext).inforef;
  const data = useContext(GeneralContext).data;

  return (
    <div className="sidebar">
      <div className="center">
        <h1>{eref.current && eref.current.children[0].innerHTML}</h1>
        {data && (data.open 
          ? <button className="clopen" onClick = {() => {data.open=false;forceupdate()}}>Close</button>
          : <button className="clopen" onClick = {() => {data.open=true;forceupdate()}}>Open</button>
        )}
      </div>
    </div>
  )
}

async function fetch_list(path) {
  let res = await fetch(`${window.location.href}list${path.length == 0 ? "" : "?path="}${path.join("/")}`);

  if (res.ok) {
    return await res.text().then((t) => t.split('\n'));
  } else {
    return "Data could not be loaded..."
  }
}

function Entry({ children, root }) {
  const ref = useRef(null);
  const setinforef = useContext(GeneralContext).setinforef;
  const setdata = useContext(GeneralContext).setdata;
  const data = useContext(GeneralContext).data;
  const forceupdate = useContext(GeneralContext).forceupdate;

  if (root.open && !root.open_ready) {
    const list = fetch_list(root.path);
    console.log(list);

    list.then((children) => {
      let i = 0;
      children.forEach((child) => {
        root.children[i++] = (new TreeNode(child, [], false, false, [...root.path, child]));
      })

      root.open_ready = true;

      forceupdate();
    });
  }

  return (<div ref={ref}>
    <button className={root.selected ? "selected" : "entry"} onClick = {() => {root.selected=true;if (data) {data.selected=false};setinforef(ref);setdata(root)}}>
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
