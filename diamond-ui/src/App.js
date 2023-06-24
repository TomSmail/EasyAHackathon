import "./App.css";
import { Footbar } from "./components/Footbar/Footbar";
import { NavBar } from "./components/NavBar/NavBar";
import { Title } from "./components/Title/Title";

function App() {
  return (
    <div className="App">
      <NavBar />
      <Title />
      <Footbar />
    </div>
  );
}

export default App;
