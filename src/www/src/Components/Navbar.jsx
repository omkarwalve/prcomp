import React, { useState } from 'react';
import { FaSearch } from "react-icons/fa";
import { FaUserCircle } from "react-icons/fa";
import {useHistory} from 'react-router-dom';
//import { FaShoppingCart } from "react-icons/fa";
//import axios from 'axios';

import './Navbar.css';
import { ReactComponent as Cart } from "./nav/cart.svg";

function Navbar() {
    const categories = [
        {
            name:'Electronics',
            url:'elx'
        },
        {
            name:'Clothing',
            url:'clo'
        },
        {
            name:'Furniture',
            url:'fur'
        }
    ]
    const history = useHistory();
    const [selectedCategory,setSelectedCategory] = useState(categories[0]);
    const [searchWord,setSearchWord] = useState('');
        const handleCategoryChange = (e) => {
            e.preventDefault();
            const {value} = e.target;
            console.log(categories.filter(elem => elem.url === value)[0]);
            setSelectedCategory(categories.filter(elem => elem.url === value)[0]);
        }
        const handleSearch = (e) => {
            const searchUrl = searchWord.split(/\s+/).join('+');
            if (searchUrl.length !== 0) {
            const {name,url} = selectedCategory;
            history.push(`/results?cat=${url}&search=${searchUrl}`);
            }
        }
    const handleKeyDown = (e) => {
        if (e.key === "Enter") { handleSearch() }
    }
                    //<p className="siteName">{websiteName}</p>
                    //<img  className="shopcart" src="/shopping_cart.svg" alt="error" />
                    //<div className="userSigning" > </div>
            return (
            <div className="navBar">
                <div className="nav-flex">
                    <div className="logo-section" onClick={() => history.push('/')}>
                        <img className="logo" src="/_kilowog_.svg" alt=""/>
                    </div>
                    <div className="search-bar-group">
                        <select className="category-selector" onChange={handleCategoryChange}>
                            {
                                categories.map(({name,url}) => {
                                    return (
                                        <option className="category-option" value={url} name={name}>{name}</option>
                                        )
                                    })
                                }
                        </select>
                       <input className="search-bar" type="text" size="55" placeholder="Search for..." value={searchWord} onChange={(e) => setSearchWord(e.target.value)} onKeyDown={handleKeyDown} />
                        <button className="search-button" onClick={handleSearch} ><FaSearch/></button>
                    </div>
                     <div className="user-options">
                         <div>
                         </div>
                        <div className="tooltip">
                            <Cart className="shopcart"/>
                            <span className="tooltip_msg">Empty</span>
                        </div>
                         <div className="dropMenu">
                         <FaUserCircle className="user-icon"/> 
                             <div className="dropMenuContent">
                                 <a href="#">My Account</a>
                                 <a href="#">Orders</a>
                                 <a href="#">Wishlist</a>
                             </div>
                         </div>
                     </div>
            </div>
        </div>
    )
}
export default Navbar;
