import "./App.css";
import { Footbar } from "./components/Footbar/Footbar";
import { Map } from "./components/Map/Map";
import { NavBar } from "./components/NavBar/NavBar";
import { SearchBar } from "./components/SearchBar/SearchBar";
// import { Title } from "./components/Title/Title";

function App() {
  return (
    <div className="App">
      <NavBar />
      <SearchBar />
      <Map />
      <Footbar />
    </div>
  );
}

export default App;
