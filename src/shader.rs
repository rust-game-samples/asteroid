use gl;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use crate::math::Matrix4;

pub struct Shader {
    shader_program: u32,
    vertex_shader: u32,
    frag_shader: u32,
}

impl Shader {
    pub fn new() -> Self {
        Self {
            shader_program: 0,
            vertex_shader: 0,
            frag_shader: 0,
        }
    }

    pub fn load(&mut self, vert_name: &str, frag_name: &str) -> Result<(), String> {
        // 頂点シェーダーとフラグメントシェーダーをコンパイル
        self.vertex_shader = self.compile_shader(vert_name, gl::VERTEX_SHADER)?;
        self.frag_shader = self.compile_shader(frag_name, gl::FRAGMENT_SHADER)?;

        // シェーダープログラムを作成し、シェーダーをリンク
        self.shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(self.shader_program, self.vertex_shader);
            gl::AttachShader(self.shader_program, self.frag_shader);
            gl::LinkProgram(self.shader_program);
        }

        // リンクが成功したか確認
        if !self.is_valid_program() {
            return Err("Failed to link shader program".into());
        }

        Ok(())
    }

    pub fn unload(&mut self) {
        // シェーダープログラムとシェーダーの削除
        unsafe {
            gl::DeleteProgram(self.shader_program);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.frag_shader);
        }
    }

    pub fn set_active(&self) {
        // このシェーダープログラムをアクティブに設定
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }

    pub fn set_matrix_uniform(&self, name: &str, matrix: &Matrix4) {
        // 行列のユニフォームを設定
        let c_name = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.shader_program, c_name.as_ptr()) };
        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::TRUE, matrix.as_ptr());
        }
    }

    fn compile_shader(&self, file_name: &str, shader_type: u32) -> Result<u32, String> {
        // シェーダーソースコードをファイルから読み込み
        let mut file = File::open(file_name).map_err(|_| format!("Failed to open shader file: {}", file_name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|_| format!("Failed to read shader file: {}", file_name))?;
        let c_contents = CString::new(contents).unwrap();

        // シェーダーオブジェクトの作成とコンパイル
        let shader = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(shader, 1, &c_contents.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }

        // コンパイルが成功したか確認
        if !self.is_compiled(shader) {
            return Err(format!("Failed to compile shader: {}", file_name));
        }

        Ok(shader)
    }

    fn is_compiled(&self, shader: u32) -> bool {
        let mut status = gl::FALSE as i32;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        }

        if status != gl::TRUE as i32 {
            let mut buffer = vec![0; 512];
            unsafe {
                gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            }
            let message = String::from_utf8_lossy(&buffer);
            eprintln!("GLSL Compile Failed: {}", message);
            return false;
        }

        true
    }

    fn is_valid_program(&self) -> bool {
        let mut status = gl::FALSE as i32;
        unsafe {
            gl::GetProgramiv(self.shader_program, gl::LINK_STATUS, &mut status);
        }

        if status != gl::TRUE as i32 {
            let mut buffer = vec![0; 512];
            unsafe {
                gl::GetProgramInfoLog(self.shader_program, 512, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
            }
            let message = String::from_utf8_lossy(&buffer);
            eprintln!("GLSL Link Status: {}", message);
            return false;
        }

        true
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.unload();
    }
}
