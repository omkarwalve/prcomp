//   __  ___     __  __      
//  /__`|__  /\ |__)/  `|__| 
//  .__/|___/~~\|  \\__,|  | 
//                           

// Library Imports
import React, { memo, useState } from 'react';

// File Imports
import { FaSearch } from "react-icons/fa";
import './search.css';

function Search({queryHolder}) {
    const plhold = "Search for..."
    const [query, setQuery] = useState('');

    const updateQuery = (e) => {
        queryHolder(query);
    }

    const handleKeyDown = (e) => {
        if (e.key === "Enter") { updateQuery(e) }
    }
    return(
        <div className="search-container">
            <input className="search-input"
                   type="search"
                   placeholder={plhold}
                   size="37"
                   required={true}
                   onChange={(e) => setQuery(e.target.value)}
                   onKeyDown={handleKeyDown}
            />
            <button className="search-button" onClick={updateQuery}>
                <FaSearch />
            </button>
        </div>
    )
}

export default memo(Search)