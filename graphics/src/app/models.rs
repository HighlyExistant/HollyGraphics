use ash::vk;
use drowsed_math::linear::{FVec3, FVec2};
use crate::buffer;
use crate::holly_types::vertex::{Vertex3DRGB, Vertex3DTexture};
use crate::{holly_types::{vertex::{self}, self}, device, buffer::{raw::Buffer}};
pub struct Model2D {
    pub vertices: Vec<vertex::Vertex2D>,
    pub indices: Vec<u32>,
}

impl holly_types::model::Mesh<vertex::Vertex2D, u32> for Model2D {
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertices(&self) -> Vec<vertex::Vertex2D> {
        self.vertices.clone()
    }
}

#[derive(Debug)]
pub struct Model3D<T: Clone> {
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
}
impl<T: Clone> Model3D<T> {
    pub fn create(&self, device: std::sync::Arc<device::Device> ) -> (buffer::raw::Buffer<T>, buffer::raw::Buffer<u32>) {
        let vertex_buffer = buffer::raw::Buffer::<T>::from_vec(device.clone(), 
            vk::BufferUsageFlags::VERTEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            self.vertices.clone()
        );
        let index_buffer = buffer::raw::Buffer::<u32>::from_vec(device.clone(), 
            vk::BufferUsageFlags::INDEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            self.indices.clone()
        );
        (vertex_buffer, index_buffer)
    }
}

impl<T: Clone> holly_types::model::Mesh<T, u32> for Model3D<T> {
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertices(&self) -> Vec<T> {
        self.vertices.clone()
    }
}

// credit to: https://pastebin.com/4T10MFgb
pub fn create_cube() -> Model3D<Vertex3DRGB> {
    Model3D{ vertices: vec![
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5, z: -0.5    },  rgb: FVec3 {x: 1.0, y: 0.0, z: 0.0}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,  z: 0.5     },  rgb: FVec3 {x: 1.0, y: 0.0, z: 0.0}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5, z: 0.5     },  rgb: FVec3 {x: 1.0, y: 0.0, z: 0.0}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,  z: -0.5    },  rgb: FVec3 {x: 1.0, y: 0.0, z: 0.0}},
   
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: -0.5, z: -0.5    },  rgb: FVec3 {x: 0.0, y: 1.0, z: 0.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: 0.5,  z: 0.5     },  rgb: FVec3 {x: 0.0, y: 1.0, z: 0.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: -0.5, z: 0.5     },  rgb: FVec3 {x: 0.0, y: 1.0, z: 0.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: 0.5,  z: -0.5    },  rgb: FVec3 {x: 0.0, y: 1.0, z: 0.0}},
   
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5, z: -0.5    },  rgb: FVec3 {x: 0.0, y: 0.0, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: -0.5, z: 0.5     },  rgb: FVec3 {x: 0.0, y: 0.0, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5, z: 0.5     },  rgb: FVec3 {x: 0.0, y: 0.0, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: -0.5, z: -0.5    },  rgb: FVec3 {x: 0.0, y: 0.0, z: 1.0}},
   
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,  z: -0.5    },  rgb: FVec3 {x: 1.0, y: 1.0, z: 0.1}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: 0.5,  z: 0.5     },  rgb: FVec3 {x: 1.0, y: 1.0, z: 0.1}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,  z: 0.5     },  rgb: FVec3 {x: 1.0, y: 1.0, z: 0.1}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: 0.5,  z: -0.5    },  rgb: FVec3 {x: 1.0, y: 1.0, z: 0.1}},
   
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5,  z: 0.5     },  rgb: FVec3 {x: 0.1, y: 1.0, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: 0.5,   z: 0.5     },  rgb: FVec3 {x: 0.1, y: 1.0, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,   z: 0.5     },  rgb: FVec3 {x: 0.1, y: 1.0, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: -0.5,  z: 0.5     },  rgb: FVec3 {x: 0.1, y: 1.0, z: 1.0}},
   
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5,  z: -0.5    },  rgb: FVec3 {x: 1.0, y: 0.8, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: 0.5,   z: -0.5    },  rgb: FVec3 {x: 1.0, y: 0.8, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,   z: -0.5    },  rgb: FVec3 {x: 1.0, y: 0.8, z: 1.0}},
        Vertex3DRGB {coords: FVec3 {x: 0.5,  y: -0.5,  z: -0.5    },  rgb: FVec3 {x: 1.0, y: 0.8, z: 1.0}},
    ], indices: vec![0u32,  1,  2,  0,  3,  1,  4,  5,  6,  4,  7,  5,  8,  9,  10, 8,  11, 9,
    12, 13, 14, 12, 15, 13, 16, 17, 18, 16, 19, 17, 20, 21, 22, 20, 23, 21] }
  }

  pub fn create_face(rgb: FVec3, ofst: FVec3, z: f32) -> Model3D<Vertex3DRGB> {
    Model3D{ vertices: vec![
        
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: -0.5,  z    }  + ofst ,  rgb},
        Vertex3DRGB {coords: FVec3 {x: 0.5 ,  y: 0.5,   z    } + ofst ,  rgb},
        Vertex3DRGB {coords: FVec3 {x: -0.5, y: 0.5,   z    }  + ofst ,  rgb},
        Vertex3DRGB {coords: FVec3 {x: 0.5 ,  y: -0.5,  z    } + ofst ,  rgb},
   
    ], indices: vec![0u32,  1,  2,  0,  3,  1] }
  }


  pub fn create_cube_textured(index: u32) -> Model3D<Vertex3DTexture> {
    Model3D{ vertices: vec![
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: -0.5, z: -0.5    },  text_coords: FVec2 {x: 0.0, y: 0.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: 0.5,  z: 0.5     },  text_coords: FVec2 {x: 1.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: -0.5, z: 0.5     },  text_coords: FVec2 {x: 0.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: 0.5,  z: -0.5    },  text_coords: FVec2 {x: 1.0, y: 0.0,    }   },
        
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: -0.5, z: -0.5    },  text_coords: FVec2 {x: 0.0, y: 0.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: 0.5,  z: 0.5     },  text_coords: FVec2 {x: 1.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: -0.5, z: 0.5     },  text_coords: FVec2 {x: 0.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: 0.5,  z: -0.5    },  text_coords: FVec2 {x: 1.0, y: 0.0,    }   },
   
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: -0.5, z: -0.5    },  text_coords: FVec2 {x: 0.0, y: 0.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: -0.5, z: 0.5     },  text_coords: FVec2 {x: 1.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: -0.5, z: 0.5     },  text_coords: FVec2 {x: 0.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: -0.5, z: -0.5    },  text_coords: FVec2 {x: 1.0, y: 0.0,    }   },
   
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: 0.5,  z: -0.5    },  text_coords: FVec2 {x: 0.0, y: 0.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: 0.5,  z: 0.5     },  text_coords: FVec2 {x: 1.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: 0.5,  z: 0.5     },  text_coords: FVec2 {x: 0.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: 0.5,  z: -0.5    },  text_coords: FVec2 {x: 1.0, y: 0.0,    }   },
   
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: -0.5,  z: 0.5     },  text_coords: FVec2 {x: 0.0, y: 0.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: 0.5,   z: 0.5     },  text_coords: FVec2 {x: 1.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: 0.5,   z: 0.5     },  text_coords: FVec2 {x: 0.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: -0.5,  z: 0.5     },  text_coords: FVec2 {x: 1.0, y: 0.0,    }   },
   
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: -0.5,  z: -0.5    },  text_coords: FVec2 {x: 0.0, y: 0.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: 0.5,   z: -0.5    },  text_coords: FVec2 {x: 1.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: -0.5, y: 0.5,   z: -0.5    },  text_coords: FVec2 {x: 0.0, y: 1.0,    }   },
        Vertex3DTexture {coords: FVec3 {x: 0.5,  y: -0.5,  z: -0.5    },  text_coords: FVec2 {x: 1.0, y: 0.0,    }   },
    ], indices: vec![0u32,  1,  2,  0,  3,  1,  4,  5,  6,  4,  7,  5,  8,  9,  10, 8,  11, 9,
    12, 13, 14, 12, 15, 13, 16, 17, 18, 16, 19, 17, 20, 21, 22, 20, 23, 21] }
  }
