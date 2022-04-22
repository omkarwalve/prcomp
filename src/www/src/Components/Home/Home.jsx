import Catalog from './Catalog';
import MiniListings from './RList';
import './home.css'


function Homepage() {
  document.title = "kilowog - The Product Comparison Engine";
  return (
		<>
			<Catalog />
			<hr className='seperator'/>
			<MiniListings uid={0} />
		</>
    );
}

export default Homepage;
