import Catalog from './Catalog';
import MiniListings from './RList';
import './home.css'


function Homepage() {
  document.title = "Comparbro- The Product Comparison Engine";
  return (
		<>
			<Catalog />
			<hr className='seperator'/>
			<MiniListings uid={1001} />
		</>
    );
}

export default Homepage;
