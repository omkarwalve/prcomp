
//   __ _   __   _  _  ____   __   ____ 
//  (  ( \ / _\ / )( \(  _ \ / _\ (  _ \
//  /    //    \\ \/ / ) _ (/    \ )   /
//  \_)__)\_/\_/ \__/ (____/\_/\_/(__\_)
  
// Library Imports
import React, { memo, useContext, useEffect, useRef, useState } from 'react';
import {useHistory} from 'react-router-dom';

// Component Imports
import Select from 'Components/Assets/Select/Select';
import Search from 'Components/Assets/Search/Search';
import Tooltip from 'Components/Assets/Tooltip/Tooltip';
import Menu from 'Components/Assets/Menu/Menu';
import { Cart } from 'Components/Assets/Cart/Cart';
import './Navbar.css';
// SVG Imports
import { ReactComponent as Kilowog } from "assets/kilowog.svg";
import { ReactComponent as Analytics } from "./assets/analytics.svg";
//import { ReactComponent as Cart } from "./assets/cart.svg";
import { ReactComponent as User } from "./assets/user.svg";
import { ProductCart, ShowLogin } from 'App';


function Navbar() {
    // Search Bar Group
    const categories = [
        {
            key:'Electronics',
            value:'elx'
        },
        {
            key:'Clothing',
            value:'clo'
        },
        {
            key:'Furniture',
            value:'fur'
        }
    ]
    const cartItems = useContext(ProductCart)[0];
    const history = useHistory();
    const [selectedCategory,setSelectedCategory] = useState(null);
    const [searchWord,setSearchWord] = useState('');
    useEffect(() => {
        const searchUrl = searchWord.split(/\s+/).join('+');
        if (searchUrl.length !== 0) {
        const { _ , value } = selectedCategory;
        history.push(`/results?cat=${value}&search=${searchUrl}`);
        }
    },[searchWord]);
    useEffect(()=> {console.log(selectedCategory);},[selectedCategory]);


    // User Menu
    const menu = [
        {
            text: 'My Account',
            uri : '#',
			action: useContext(ShowLogin)
        },
        {
            text: 'Browse History',
            uri : '#'
        },
        {
            text: 'Watchlist Analytics',
            uri : '#'
        },
    ]
    const menuRef = useRef(null);
    const [isMenuActive,setMenuActive] = useState(false);
    const onMenuClick = () => setMenuActive(!isMenuActive);
    
    // on Mount useEffect for Menu event listener
    const pageClickEvent = (e) => {
        if (menuRef.current && !menuRef.current.contains(e.target) ) {
            setMenuActive(!isMenuActive);
        }
    }
    useEffect(()=>{
        //if (isMenuActive) { document.addEventListener('click',pageClickEvent); }
        //return () => { document.removeEventListener('click', pageClickEvent); }
        (isMenuActive) 
            ? document.addEventListener('click',pageClickEvent)
            : document.removeEventListener('click', pageClickEvent)
    },[isMenuActive]);
    return (
        <div className="navbar">
            <div className="nav-flex">
                <Kilowog className="website-logo" onClick={() => history.push('/')} />
                <div className="search-bar-group">
                    <Select items={categories} optionHolder={setSelectedCategory} />
                    <Search queryHolder={setSearchWord} />
                </div>
                <ul className="user-options" type="none">
                    <li className="user-option"><Analytics /> <Tooltip text="Analytics Coming Soon!" /></li>
                    {/*<li className="user-option"><Cart      /> <Tooltip text="0 items in cart" /></li>*/}
                    <li className="user-option"><Cart cart={cartItems}/></li>
                    <li ref={menuRef}
                        onClick={onMenuClick}
                        className="user-option"><User      />
                            <Menu items={menu} menuSwitch={isMenuActive}>
                                <h1>hello</h1>
                            </Menu>
                        </li>
                </ul>
            </div>
        </div>
    )
}

export default memo(Navbar);
