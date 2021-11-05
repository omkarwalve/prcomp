import './App.css';
import Navbar from './Components/Navbar';
import Catalog from './Components/Catalog';


function Homepage() {
  document.title = "kilowog - The Product Comparison Engine";
  return (
    <div>
        <Catalog />
    </div>
    );
}

export default Homepage;
