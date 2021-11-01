import './App.css';
import Navbar from './Components/Navbar';
import Catalog from './Components/Catalog';
import Footer from './Components/Footer';


function Homepage() {
  return (
    <div>
      <div>
        <Catalog />
        </div>
        <div>
          <Footer />
          </div>
           
      </div>
    
    );
}

export default Homepage;
