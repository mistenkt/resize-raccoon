import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
//import "./styles.css";
import './global.css';
import './boot';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
      <App/>
  </React.StrictMode>,
);
