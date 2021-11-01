import './App.css';
import Homepage from './Homepage';
import Productlistpage from './Productlistpage';
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom'
import Navbar from './Components/Navbar';


function App() {
  return (
    <>
      <Router>
          <Navbar/>
        <Switch>
          <Route path='/' exact>
            <Homepage/>
          </Route>
          <Route path='/results'>
            <Productlistpage/>
          </Route>
        </Switch>
      </Router>
    </>
  );
}

export default App;
