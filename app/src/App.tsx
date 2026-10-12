import { Stage, Layer, Line } from "react-konva";
// Empty scaffold only: no case, sample data, persistence commands, or diagram editor.
export default function App() {
 return <div className="workspace">
  <header><span className="mark" aria-hidden="true">◇</span><strong>Risk Case Workspace</strong><span className="local">Local workspace</span></header>
  <aside aria-label="Case explorer"><h2>Case explorer</h2><p className="muted">No case open</p>
   {["Diagram groups", "Activities", "Documents", "Reference lists"].map(title => <details key={title}><summary>{title}</summary><p className="muted">No items</p></details>)}
  </aside>
  <main><div className="empty"><Stage width={180} height={80} listening={false} aria-hidden="true"><Layer><Line points={[10,40,90,40,170,40]} stroke="#8aa5a0" strokeWidth={1} /></Layer></Stage>
   <h1>A place to understand risk</h1><p>Hazards, barriers, and the work that keeps them effective.</p><span className="muted">No case open</span></div></main>
  <footer>Offline · No account required</footer>
 </div>;
}
