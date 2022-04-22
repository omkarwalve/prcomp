import React, { useReducer, useEffect, useState, useContext, createContext } from 'react';

import './App.css';
import Home from './Components/Home/Home';
import Footer from './Components/Footer/Footer';
import Listing from './Components/Listings/Listing';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom'
import Navbar from './Components/Navbar/Navbar';
import { cartActions, cartReducer } from 'Components/Assets/Cart/Cart';
import { ShortProduct } from 'Components/Listings/cogs/product';
import useObserve from 'hooks/observe';
import Login from 'Components/Assets/Login/Login';
import useToggle from 'hooks/toggle';

export const Viewport = createContext<number[]>([0,0]);
export const ProductCart = createContext<[Set<ShortProduct|unknown>,React.Dispatch<cartActions>]>([new Set(),() => {}]);
export const ShowLogin= createContext<() => void>(() => {});

function App() {
  /** `Cart Global State` */
  const [cart, setCart] = useReducer(cartReducer, new Set());
  const [login, toggleLogin ] = useToggle(false);
  useObserve(cart,"cart");

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
        <ProductCart.Provider value={[cart,setCart]}>
		<ShowLogin.Provider value={toggleLogin}>
		{(login) && (<Login />)}
        <Router>
            <Navbar />
              <main id="main-content">
                <Switch>
                  <Route path='/' exact>
                    <Home/>
                  </Route>
                  <Route path='/results'>
                      <Listing />
                  </Route>
                </Switch>
              </main>
          <Footer/>
        </Router>
		</ShowLogin.Provider>
        </ProductCart.Provider>
      </Viewport.Provider>
    </>
  );
}

export default App;
