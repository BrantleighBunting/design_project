import React, { Component, PropTypes } from 'react';
import axios from 'axios';
// import UTClass from './UTClass.jsx';

// App
export default class App extends Component {
	constructor(props) {
		super(props);
		this.state = {
			course_data: [],
			bs_data: []
		};
	}

	componentDidMount() {
		axios.get('http://localhost:3002/courses')
		.then(response => {
			this.obj_data = response.data.data[0];
			console.log(this.obj_data.ID);
			
			React.renderComponent(
				new CourseComponent({
					bs_data: response.data
				}),
				document.getElementById('course_component');
			);
		});

		axios.get('http://localhost:3002/courses')
		.then(response => {
			this.obj_data = response.data.data[0];
			console.log(this.obj_data.ID);

			React.renderComponent(
				new CourseComponent({
					course_data: response.data
				}),
				document.getElementById('course_component');
			);
			// this.setProps({
			// 	bs_data: response.data
			// });
		});
	}

	render() {
		return (
			<div className="container">
				<header>
					<h1>MyUT Schedule</h1>
				</header>
				<ul>
					{
						this.state.bs_data.map((v, index) => {
							return <li key={index}>{v.ID} {v.ACADEMIC_PERIOD} {v.COURSE_LEVEL}</li>
						})
					}
				</ul>
			</div>
		);
	}
}

var CourseComponent = React.createClass({
	getDefaultProps: () => {
		return {

		};
	},

	propTypes: {

		bs_data: React.PropTypes.array,
		course_data: React.PropTypes.array,

		ACADEMIC_PERIOD: React.PropTypes.string,
		COURSE_LEVEL: React.PropTypes.string,

		name: React.PropTypes.string,
		visible: React.PropTypes.bool.isRequired,
	},

	render: () => {
		return <div>{this.props.name}</div>;
	}
});








