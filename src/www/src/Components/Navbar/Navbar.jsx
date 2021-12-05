
//   __ _   __   _  _  ____   __   ____       __  ____ 
//  (  ( \ / _\ / )( \(  _ \ / _\ (  _ \    _(  )/ ___)
//  /    //    \\ \/ / ) _ (/    \ )   / _ / \) \\___ \
//  \_)__)\_/\_/ \__/ (____/\_/\_/(__\_)(_)\____/(____/
  
// Library Imports
import React, { useEffect, useRef, useState } from 'react';
import {useHistory} from 'react-router-dom';

// File Imports
import Select from 'Components/Assets/Select/Select';
import Search from 'Components/Assets/Search/Search';
import Tooltip from 'Components/Assets/Tooltip/Tooltip';
import Menu from 'Components/Assets/Menu/Menu';
import './navbar.css';
// Icon Imports
import { ReactComponent as Kilowog } from "assets/kilowog.svg";
import { ReactComponent as Analytics } from "./assets/analytics.svg";
import { ReactComponent as Cart } from "./assets/cart.svg";
import { ReactComponent as User } from "./assets/user.svg";


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
            uri : '#'
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
    useEffect(()=>{
        const pageClickEvent = (e) => {
            if (menuRef.current != null && !menuRef.current.contains(e.target) ) {
                setMenuActive(!isMenuActive);
            }
        }
        if (isMenuActive) {
            document.addEventListener('click',pageClickEvent);
        }
        return () => {
            document.removeEventListener('click', pageClickEvent);
        }
    },[]);
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
                    <li className="user-option"><Cart      /> <Tooltip text="0 items in cart" /></li>
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

export default Navbar
