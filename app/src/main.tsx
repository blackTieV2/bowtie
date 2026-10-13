import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./style.css";
import {loadFont} from './fontAssets';
void loadFont().then(()=>createRoot(document.getElementById("root")!).render(<React.StrictMode><App /></React.StrictMode>));
