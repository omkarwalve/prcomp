import './App.css';
import Home from './Home';
import Footer from './Components/Footer';
import Listing from './Components/Listing';
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom'
import Navbar from './Components/Navbar';


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
