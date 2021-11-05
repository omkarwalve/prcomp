
import React, { useState } from 'react';
import './Navbar.css';
import { FaSearch } from "react-icons/fa";
import { FaShoppingCart } from "react-icons/fa";
import { FaUserCircle } from "react-icons/fa";
import {useHistory} from 'react-router-dom'
//import axios from 'axios';

const websiteName = "kilowog";

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
            const {url,name} = selectedCategory;
            history.push(`/results?cat=${url}&search=${searchUrl}`);
            }
        }
    const handleKeyDown = (e) => {
        if (e.key === "Enter") { handleSearch() }
    }
                    //<p className="siteName">{websiteName}</p>
            return (
            <div className="navBar">
                <div className="logoName" onClick={() => history.push('/')}>
                    <img className="logo" src="/_kilowog_.svg" alt=""/>
                </div>
                <div className="sBarGroup">
                    <select className="categoryDropdown" onChange={handleCategoryChange}>
                        {
                            categories.map(({name,url}) => {
                                return (
                                    <option className="catOpts" value={url} name={name}>{name}</option>
                                    )
                                })
                            }
                    </select>
                   <input className="search_bar" type="text" size="55" placeholder="Search for..." value={searchWord} onChange={(e) => setSearchWord(e.target.value)} onKeyDown={handleKeyDown} />
                    <button className="search_btn" onClick={handleSearch} > <FaSearch color="gray"/></button>
                </div>
                 <div className="userOpts">
                    <img  className="shopcart" src="/shopping_cart.svg" alt="error" />
                    <div className="userSigning" ><FaUserCircle />  </div>
                 </div>

        </div>
    )
}

export default Navbar;
