import { useReducer, useEffect, useState, useContext, createContext } from 'react';

import './App.css';
import Home from './Components/Home/Home';
import Footer from './Components/Footer/Footer';
import Listing from './Components/Listings/Listing';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom'
import Navbar from './Components/Navbar/Navbar';
import { cartReducer } from 'Components/Assets/Cart/Cart';

export const Viewport = createContext<number[]>([0,0]);

function App() {
  /** `Cart Global State` */
  const [cart, setCart] = useReducer(cartReducer, new Set());
  useEffect(() => { console.log(cart); }, [cart]);

  const [winWidth, setWinWidth] = useState<number>(window.innerWidth);
  const [winHeight, setWinHeight] = useState<number>(window.innerHeight);

  useEffect(() => {
    window.addEventListener('resize', () => {
      setWinWidth(window.innerWidth);
      setWinHeight(window.innerHeight);
    });
  }, []);

  return (
    <>
      <Viewport.Provider value={[winWidth,winHeight]}>
        <Router>
            <Navbar cartItems={cart}/>
              <main id="main-content">
                <Switch>
                  <Route path='/' exact>
                    <Home/>
                  </Route>
                  <Route path='/results'>
                      <Listing setCart={setCart}/>
                  </Route>
                </Switch>
              </main>
          <Footer/>
        </Router>
      </Viewport.Provider>
    </>
  );
}

export default App;