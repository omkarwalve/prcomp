import { useReducer, useEffect } from 'react';

import './App.css';
import Home from './Components/Home/Home';
import Footer from './Components/Footer/Footer';
import Listing from './Components/Listings/Listing';
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom'
import Navbar from './Components/Navbar/Navbar';
import { cartReducer } from 'Components/Assets/Cart/Cart';


function App() {
  /** `Cart Global State` */
  const [cart, setCart] = useReducer(cartReducer, new Set());
  useEffect(()=>{ console.log(cart) },[cart]);

  return (
    <>
      <Router>
          <Navbar cartItems={cart}/>
        <Switch>
          <Route path='/' exact>
            <Home/>
          </Route>
          <Route path='/results'>
            <Listing setCart={setCart}/>
          </Route>
        </Switch>
        <Footer/>
      </Router>
    </>
  );
}

export default App;
