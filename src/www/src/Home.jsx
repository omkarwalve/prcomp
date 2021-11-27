import './App.css';
import Catalog from './Components/Catalog';


function Homepage() {
  document.title = "kilowog - The Product Comparison Engine";
  return (
        <Catalog />
    );
}

export default Homepage;
