precision mediump float;

varying vec4 v_position;

vec4 convRGBA(float depth){
    float r = depth;
    float g = fract(r * 255.0);
    float b = fract(g * 255.0);
    float a = fract(b * 255.0);
    float coef = 1.0 / 255.0;
    r -= g * coef;
    g -= b * coef;
    b -= a * coef;
    return vec4(r, g, b, a);
}

void main(void){
    float near = 0.5;
    float far  = 100.0;
    float linerDepth = 1.0 / (far - near);
    linerDepth *= v_position.z / v_position.w;
    vec4 convColor = convRGBA(linerDepth);
    gl_FragColor = convColor;
}
