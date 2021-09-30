import './App.css';
import Section from './Components/Section';
import Navbar from './Components/Navbar';
import Footer from './Components/Footer';
;


function Homepage() {
  return (
    <div>
      <Navbar/>
      <div>
        <Section />
        </div>
        <div>
          <Footer />
          </div> 
      </div>
    
    );
}

export default Homepage;
