import './App.css';
import Home from './Components/Home/Home';
import Footer from './Components/Footer/Footer';
import Listing from './Components/Listings/Listing';
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom'
import Navbar from './Components/Navbar/Navbar';


function App() {
  return (
    <>
      <Router>
          <Navbar/>
        <Switch>
          <Route path='/' exact>
            <Home/>
          </Route>
          <Route path='/results'>
            <Listing/>
          </Route>
        </Switch>
        <Footer/>
      </Router>
    </>
  );
}

export default App;
