use crate::actor::Actor;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::math::Matrix4;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SpriteComponent {
    owner: Rc<RefCell<Actor>>,
    texture: Option<Rc<RefCell<Texture>>>,
    draw_order: i32,
    tex_width: i32,
    tex_height: i32,
}

impl SpriteComponent {
    pub fn new(owner: Rc<RefCell<Actor>>, draw_order: i32) -> Self {
        let sprite_component = Self {
            owner,
            texture: None,
            draw_order,
            tex_width: 0,
            tex_height: 0,
        };

        let game = sprite_component.owner.borrow_mut().get_game();
        game.borrow_mut().add_sprite(Rc::new(RefCell::new(sprite_component.clone())));

        sprite_component
    }

    pub fn draw(&self, shader: &Shader) {
        if let Some(texture) = &self.texture {
            // テクスチャの幅と高さに基づいてスケーリング行列を生成
            let scale_mat = Matrix4::create_scale(
                self.tex_width as f32,
                self.tex_height as f32,
                1.0,
            );

            // ワールド変換行列を取得してスケーリング行列と掛け合わせる
            let world = scale_mat * self.owner.borrow_mut().get_world_transform();

            // シェーダーにワールド変換行列を設定
            shader.set_matrix_uniform("uWorldTransform", &world);

            // テクスチャをアクティブにする
            texture.borrow_mut().set_active();

            // 四角形を描画
            unsafe {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            }
        }
    }

    pub fn set_texture(&mut self, texture: Rc<RefCell<Texture>>) {
        self.texture = Some(texture.clone());
        self.tex_width = texture.borrow_mut().get_width() as i32;
        self.tex_height = texture.borrow_mut().get_height() as i32;
    }

    pub fn get_draw_order(&self) -> i32 {
        self.draw_order
    }

    pub fn get_tex_height(&self) -> i32 {
        self.tex_height
    }

    pub fn get_tex_width(&self) -> i32 {
        self.tex_width
    }
}

impl Drop for SpriteComponent {
    fn drop(&mut self) {
        let sprite_component = Rc::new(RefCell::new(self.clone()));
        self.owner.borrow_mut().get_game().borrow_mut().remove_sprite(sprite_component);
    }
}
