#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

void main() {
    //Color is the color we receive from ColorMaterial's color field, we send it to the fragment shader from the vertex shader
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    //Get the color of the current fragment from the texture
    vec4 texture_pixel_color =  texture(
        sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
        v_Uv);
    //TODO: this should be relative to the scale, make a new shader material
    if(v_Uv.x < 0.01 || v_Uv.y < 0.01 || v_Uv.x > 0.99 || v_Uv.y > 0.99){
        color *= vec4(0.0,0.0,0.0,1.0);
        
    }
    else{
        //If the color is transparent
        if(texture_pixel_color.a == 0.0){
            //Get the fragment's pixel position in the texture
            vec2 pixel = v_Uv * textureSize(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),0);
            
            //If the pixel's position fits in the equation y=x+b where b%2==0  (a straight line of pixels from every even row, creates a checkerboard pattern)
            if(mod(int(pixel.y)-int(pixel.x),2.0)==0.0){
                //Color the fragment slightly white
                texture_pixel_color = vec4(0.502,0.502,0.502,1.0);
            }
            else{
                //Color the fragment even whiter
                texture_pixel_color = vec4(0.802,0.802,0.802,1.0);
            }
        }
        color *= texture_pixel_color;
    }
    //Mutliply the ColorMaterial's color with: 
    //  If the alpha is zero, by our chckerboard pattern
    //  If the alpha is non-zero, by the texture's pixel color
    
# endif
    o_Target = color;
}