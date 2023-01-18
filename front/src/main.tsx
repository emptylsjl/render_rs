
import React from 'react';
import ReactDOM from 'react-dom/client';
import reportWebVitals from './reportWebVitals';

import './layout.css'
import App from "./App";
import RPanel from "/./RMPanel/RPanel";
import NavMainPanel from "./NavBarPanel/NavPanel";
import MContentPanel from "./MainContentPanel/MCPanel";

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App/>
        <MContentPanel/>
        <NavMainPanel/>
        <RPanel/>
    </React.StrictMode>,
)

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
