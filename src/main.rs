pub mod imports;

use fltk::app::event_coords;
use fltk::dialog;
use fltk::output::*;
#[warn(unused_assignments)]
use fltk::{app, window::Window, prelude::*, button::*, frame::Frame, draw::*, enums::*, input, group, image};
use imports::*;
use num::range;
use num::traits::Pow;
use std::rc::*;
use std::cell::*;
// use std::error::*;

const HEIGHT: i32 = 900;
const WIDTH: i32 = 1600;
const OUTER: i32 = 10;

fn main() {
	let app = app::App::default().with_scheme(app::Scheme::Gtk);
	let mut win = Window::default()
		.with_size(WIDTH, HEIGHT)
		.center_screen()
		.with_label("Polynomial Size Calculate");
	win.make_resizable(true);
	let main_frame = Rc::from(RefCell::from(Frame::new(OUTER, OUTER * 3, WIDTH - OUTER * 2, HEIGHT - OUTER * 4, "")));
	main_frame.borrow_mut().set_color(Color::White);
	// main_frame.
	main_frame.borrow_mut().set_frame(FrameType::ThinDownBox);
	let mut file_but = Button::default().with_label("File").with_size(40, 27).with_pos(5, 0);
	let flex1 = group::Flex::default().with_size(250, 27).with_pos(50, 0).row();
	// flex.set_margins(OUTER, OUTER, 50 + OUTER, 100 + OUTER);

	// let mut frame = Frame::new(OUTER, OUTER, HEIGHT - OUTER * 2, WIDTH - OUTER * 2, "");
	let mut frame = Frame::default().with_label("Enter Scale");
	let inp = input::IntInput::default();
	let mut but = Button::default().with_label("Submit");

	frame.set_color(Color::DarkCyan);
	frame.set_frame(FrameType::EngravedBox);
	flex1.end();

	let image_actual_ratio = Rc::from(RefCell::from(100));
	let iar = image_actual_ratio.clone();
	but.set_callback(move |_| {
		*iar.borrow_mut() = inp.value().parse::<i32>().unwrap();
		println!("answer: {}", *iar.borrow());
	});

	let dialog = Rc::from(
		RefCell::from(
			dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile)
			// dialog::FileChooser::new(
			// 	".",
			// 	"*.{jpg, jpeg, svg, png}",
			// 	dialog::FileChooserType::Single,
			// 	"Select a map"
			// )
		)
	);
	dialog.borrow_mut().set_filter("*.{jpg,jpeg,png,svg}");
	// let file_path = Rc::from(RefCell::from(Path::new("")));
	// let file_path_pt = file_path.clone();
	// let file_str_pt2 = file_str.clone();
	let file_flag = Rc::from(RefCell::from(false));
	let file_flag_cp = file_flag.clone();
	let dialog_pt = dialog.clone();
	let fr_pt = main_frame.clone();
	file_but.set_callback(move |_| {
		dialog_pt.borrow_mut().show();
		println!("{:?}", dialog_pt.borrow().filename());
		*file_flag_cp.borrow_mut() = true;
		fr_pt.borrow_mut().redraw();
		// file_path_pt.as_ref().borrow_mut() = dialog.filename().as_path();//.into_os_string().into_string().unwrap();
	});


	let mut coor_frame = Frame::default().with_label("Coordinates").with_size(160, 27).with_pos(320, 0);
	let out_coor_pt = Rc::from(RefCell::from(Output::default().with_size(305, 27).with_pos(485, 0)));
	coor_frame.set_color(Color::Magenta);
	coor_frame.set_frame(FrameType::DownBox);
	let mut size_frame = Frame::default().with_label("Area").with_size(50, 27).with_pos(810, 0);
	size_frame.set_color(Color::from_rgb(30, 232, 124));
	size_frame.set_frame(FrameType::DownBox);
	let mut out_size = Output::default().with_size(105, 27).with_pos(865, 0);
	let mut clear_but = Button::default().with_label("Clear Selected Points").with_size(200, 27).with_pos(1000, 0);
	let mut scale_but = Button::default().with_label("Click Scale").with_size(160, 27).with_pos(1210, 0);
	let mut helpm_but = Button::default().with_label("Help").with_size(70, 27).with_pos(1380, 0);
	// // let image = image::JpegImage::load(dialog.borrow().filename().as_path()).unwrap();
	// let mut image = image::JpegImage::load("D:\\bin\\好看的照骗\\微信图片_20231028011138.jpg").unwrap();
	// image.scale(main_frame.w(), main_frame.h(), true, true);
	// image.scale()
	// main_frame.set_image(Some(image));

	// let flex2 = group::Flex::default().with_size(500, 27).with_pos(320, 0).row();
	// flex2.end();

	let poly = Rc::from(RefCell::from(Polygon::new()));

	// .with_pos can set position!
	let coor_string = Rc::from(RefCell::from(String::from("")));
	let coor_str = coor_string.clone();
	let finished_pt = Rc::from(RefCell::from(false));
	let scale_pt = Rc::from(RefCell::from(-1));
	let scale_x = Rc::from(RefCell::from(Vec2d::new(-1.0, -1.0)));
	let scale_y = Rc::from(RefCell::from(Vec2d::new(-1.0, -1.0)));
	// let mut finished_pt = false;
	out_coor_pt.borrow_mut().set_value(&coor_str.borrow());
	win.end();
	win.show();

	let scale_pt_cp = scale_pt.clone();
	let sx = scale_x.clone();
	let sy = scale_y.clone();
	scale_but.set_callback(move |_| {
		*scale_pt_cp.borrow_mut() = 0;
		*sx.borrow_mut() = Vec2d::new(-1.0, -1.0);
		*sy.borrow_mut() = Vec2d::new(-1.0, -1.0);
	});

	helpm_but.set_callback(move |_| {
		dialog::message_title("Help");
		dialog::message_default(
			"Click mouse left button to add points.\nClick mouse right button to stop adding points or to refresh.\nClick \"Click Scale\" to point the two ends of the plotting scale.\nAfter selecting the polygon vertices and plotting scale points, enter the value of plotting, click \"Submit\" and click the mouse right button again, and the result will show in the output box beside the \"Area\" box.\nIf you can't add vertices to the polygon, try click the \"Clear Selected Points\" and try again. "
		);
	});

	// let offs = Offscreen::new(main_frame.width(), main_frame.height()).unwrap();
	// // offs.begin();
	// // draw_rect_fill(0, 0, main_frame.width(), main_frame.height(), Color::White);
	// // offs.end();

	// let offs = Rc::from(RefCell::from(offs));
	let finished = finished_pt.clone();
	let poly_out = poly.clone();
	let sx = scale_x.clone();
	let sy = scale_y.clone();
	main_frame.borrow_mut().draw({
	//     let offs = offs.clone();
	    move |f| {
	// 	let mut offs = offs.borrow_mut();
	// 	if offs.is_valid() {
	// 	    offs.rescale();
	// 	    offs.copy(OUTER, OUTER * 3, WIDTH - 2*OUTER, HEIGHT - 4*OUTER, 0, 0);
	// 	} else {
	// 	    offs.begin();
	// 	    draw_rect_fill(0, 0, WIDTH - 2*OUTER, HEIGHT - 4*OUTER, Color::White);
	// 	//     draw_rect(0, 0, WIDTH - 2*OUTER, HEIGHT - 4*OUTER);
	// 	    offs.copy(OUTER, OUTER * 3, WIDTH - OUTER*2, HEIGHT - OUTER*4, 0, 0);
	// 	    offs.end();
	// 	}
		if *file_flag.borrow() == true {
			if dialog.borrow().filename().extension().unwrap().to_str() == Some("jpg") || dialog.borrow().filename().extension().unwrap().to_str() == Some("jpeg") {
				let mut image = image::JpegImage::load(dialog.borrow().filename().as_path()).unwrap();
				image.scale(f.w(), f.h(), true, true);
				image.draw(f.x(), f.y(), f.w(), f.h());
				println!("jpg");
			}
			else if dialog.borrow().filename().extension().unwrap().to_str() == Some("png") {
				let mut image = image::PngImage::load(dialog.borrow().filename().as_path()).unwrap();
				image.scale(f.w(), f.h(), true, true);
				image.draw(f.x(), f.y(), f.w(), f.h());
				println!("png");
			}
			else if dialog.borrow().filename().extension().unwrap().to_str() == Some("svg") {
				let mut image = image::SvgImage::load(dialog.borrow().filename().as_path()).unwrap();
				image.scale(f.w(), f.h(), true, true);
				image.draw(f.x(), f.y(), f.w(), f.h());
				println!("svg");
			}
		}
		let poly_mut = poly_out.clone();
		let l = poly_mut.borrow().len();
		for i in range(0, l) {
			// println!("Draw damn point");
			// draw_point(poly_mut.borrow()[i as i32].x as i32, poly_mut.borrow()[i as i32].y as i32);
			set_draw_color(Color::from_rgb(100, 200, 45));
			set_line_style(LineStyle::Solid, 3);
			draw_circle(poly_mut.borrow()[i as i32].x as f64, poly_mut.borrow()[i as i32].y as f64, 5.0);
		}
		if *finished.borrow() == true {
			// println!("Draw");
			poly_mut.borrow().print();
			set_draw_color(Color::from_rgb(150, 255, 155));
			set_line_style(LineStyle::Solid, 3);
			begin_complex_polygon();
			let l = poly_mut.borrow().len();
			for i in range(0, l) {
				// println!("Draw damn point");
				// draw_point(poly_mut.borrow()[i as i32].x as i32, poly_mut.borrow()[i as i32].y as i32);
				vertex(poly_mut.borrow()[i as i32].x, poly_mut.borrow()[i as i32].y);
			}
			end_complex_polygon();
		}
		if sx.borrow().x != -1.0 && sx.borrow().y != -1.0 {
			set_draw_color(Color::from_rgb(235, 82, 22));
			set_line_style(LineStyle::Solid, 2);
			draw_circle(sx.borrow().x as f64, sx.borrow().y as f64, 3.0);
		}
		if sy.borrow().x != -1.0 && sy.borrow().y != -1.0 {
			set_draw_color(Color::from_rgb(235, 82, 22));
			set_line_style(LineStyle::Solid, 2);
			draw_circle(sy.borrow().x as f64, sy.borrow().y as f64, 3.0);
		}
		if sx.borrow().x != -1.0 && sx.borrow().y != -1.0 && sy.borrow().x != -1.0 && sy.borrow().y != -1.0 {
			set_draw_color(Color::from_rgb(235, 82, 22));
			set_line_style(LineStyle::Solid, 2);
			draw_line(sx.borrow().x as i32, sx.borrow().y as i32, sy.borrow().x as i32, sy.borrow().y as i32);
		}
	// 	// offs.begin();
	// 	// let _ = draw_image(&image.to_rgb_data(), 0, 0, image.w(), image.h(), image.depth());
	// 	// offs.end();
	    }
	});
	let poly_out = poly.clone();
	let finished = finished_pt.clone();
	let out_coor = out_coor_pt.clone();
	let sx = scale_x.clone();
	let sy = scale_y.clone();
	main_frame.borrow_mut().handle(move |f, env| {
		// let offs_hd = offs.borrow_mut();
		match env {
			Event::Push => {
				if app::event_mouse_button() == app::MouseButton::Left {
					println!("Click True");
					if *scale_pt.borrow() != -1 && *scale_pt.borrow() < 2 {
						let x = event_coords().0;
						let y = event_coords().1;
						println!("Get Scale: ({}, {})", x, y);
						if *scale_pt.borrow() == 0 {
							*sx.borrow_mut() = Vec2d::new(x as f64, y as f64);
						}
						else {
							*sy.borrow_mut() = Vec2d::new(x as f64, y as f64);
						}
						*scale_pt.borrow_mut() += 1;
						set_draw_color(Color::from_rgb(235, 82, 22));
						set_line_style(LineStyle::Solid, 2);
						draw_circle(x as f64, y as f64, 3.0);
						f.redraw();

						return true;
					}
					if *scale_pt.borrow() == 2 {
						*scale_pt.borrow_mut() = -1;
					}
					if *finished.borrow() == true {
						return true;
					}
					// offs_hd.begin();
					let x = event_coords().0;// - f.x();
					let y = event_coords().1;// - f.y();
					let mut _coorstr = coor_str.borrow_mut();
					let mut sss = x.to_string();
					sss.push(',');
					sss += &y.to_string();
					sss.push(';');
					*_coorstr += &sss;
					out_coor.borrow_mut().set_value(&*_coorstr);
					// out_coor.set_value(&coor_str.borrow());
					poly.borrow_mut().add_point(x as f64, y as f64);
		
					set_draw_color(Color::from_rgb(100, 200, 45));
					set_line_style(LineStyle::Solid, 3);
					draw_circle(x as f64, y as f64, 5.0);
					// offs_hd.end();
					f.redraw();
				}
				else {
					// offs_hd.begin();
					if sx.borrow().x == -1.0 || sx.borrow().y == -1.0 || sy.borrow().x == -1.0 || sy.borrow().y == -1.0 {
						dialog::alert_default("You haven't chosen a scale yet!");
					}
					println!("Right Click");
					let poly_mut = poly.clone();
					println!("Draw");
					poly_mut.borrow().print();
					set_draw_color(Color::from_rgb(150, 255, 155));
					set_line_style(LineStyle::Solid, 3);
					begin_complex_polygon();
					let l = poly_mut.borrow().len();
					for i in range(0, l) {
						// println!("Draw damn point");
						// draw_point(poly_mut.borrow()[i as i32].x as i32, poly_mut.borrow()[i as i32].y as i32);
						vertex(poly_mut.borrow()[i as i32].x, poly_mut.borrow()[i as i32].y);
					}
					end_complex_polygon();
					// offs_hd.end();
					f.redraw();
					let length_sqr = (*sx.borrow() - *sy.borrow()).sqr_length();
					let result: f64 = poly.borrow().get_size() * (*image_actual_ratio.borrow() as f64).pow(2) / length_sqr;
					out_size.set_value(&result.to_string());
					*finished.borrow_mut() = true;
					// *sx.borrow_mut() = Vec2d::new(-1.0, -1.0);
					// *sy.borrow_mut() = Vec2d::new(-1.0, -1.0);
				}
				true
			}, 
			_ => false,
		}
	});

	clear_but.set_callback(move |_| {
		*finished_pt.borrow_mut() = false;
		poly_out.borrow_mut().clear();
		coor_string.borrow_mut().clear();
		out_coor_pt.borrow_mut().set_value(&*coor_string.borrow());
		main_frame.borrow_mut().redraw();
	});

	// out_size.set_value(&poly_out.borrow().get_size().to_string());


	app.run().unwrap();
	// app.run()?;
	// Ok(());
}
