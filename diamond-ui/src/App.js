import "./App.css";
import { Footbar } from "./components/Footbar/Footbar";
import { Map } from "./components/Map/Map";
import { NavBar } from "./components/NavBar/NavBar";

function App() {
  return (
    <div className="App">
      <NavBar />
      <Map />
      <Footbar />
    </div>
  );
}

export default App;
